use color_eyre::{eyre::eyre, Section};

use crate::{prelude::DeviceName, xcrun::simctl::XcRunSimctlInstance};

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DeviceSimulatorUnBooted {
	/// Automatically selects the latest iPad simulator
	#[arg(long, group = "device_name")]
	ipad: bool,

	/// Automatically selects the latest iPhone simulator
	#[arg(long, group = "device_name")]
	iphone: bool,

	/// Provide an exact name, e.g. "iPhone 12 Pro Max"
	#[arg(group = "device_name")]
	name: Option<DeviceName>,
}

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DeviceSimulatorBooted {
	/// Will find any device that is already booted
	#[arg(long, group = "device_name")]
	booted: bool,

	/// Automatically selects a booted iPad simulator
	#[arg(long, group = "device_name")]
	ipad: bool,

	/// Automatically selects a booted iPhone simulator
	#[arg(long, group = "device_name")]
	iphone: bool,

	/// Provide an exact name, e.g. "iPhone 12 Pro Max"
	/// Will confirm it is booted
	#[arg(group = "device_name")]
	name: Option<DeviceName>,
}

impl DeviceSimulatorUnBooted {
	#[tracing::instrument(level = "trace", skip(self, simctl_instance))]
	pub fn resolve(
		self,
		simctl_instance: &XcRunSimctlInstance,
	) -> Result<DeviceName, color_eyre::Report> {
		match (self.ipad, self.iphone, self.name) {
			(false, false, Some(name)) => Ok(name),
			(true, false, None) => {
				let list = simctl_instance.list()?;
				let latest_ipad = list
					.ipads()
					.max()
					.ok_or_else(|| eyre!("No simulator iPads found!"))?;
				Ok((*latest_ipad).into())
			}
			(false, true, None) => {
				let list_output = &simctl_instance.list()?;
				let latest_iphone = list_output
					.iphones()
					.max()
					.ok_or_else(|| eyre!("No simulator iPhones found!"))?;
				Ok(latest_iphone.clone().into())
			}
			_ => Err(eyre!("Clap arguments should prevent this")),
		}
	}
}

impl DeviceSimulatorBooted {
	pub fn resolve(self, simctl_instance: &XcRunSimctlInstance) -> color_eyre::Result<DeviceName> {
		let list = simctl_instance.list()?;
		let booted_devices = list.devices().filter(|d| d.ready()).collect::<Vec<_>>();
		if booted_devices.is_empty() {
			Err(eyre!("No booted devices found!").with_suggestion(|| "try running `apple-clis xcrun simctl boot` to boot a simulator"))
		} else {
			match (self.booted, self.ipad, self.iphone, self.name) {
				(true, false, false, None) => Ok(booted_devices[0].name.clone()),
				(false, true, false, None) => booted_devices
					.iter()
					.filter_map(|d| d.name.get_ipad())
					.max()
					.cloned()
					.map(DeviceName::from)
					.ok_or_else(|| eyre!("No booted iPads found!"))
					.with_suggestion(|| {
						"try running `apple-clis xcrun simctl boot --ipad` to boot a simulator"
					})
					.with_note(|| format!("Booted devices: {:?}", &booted_devices)),
				(false, false, true, None) => booted_devices
					.iter()
					.filter_map(|d| d.name.get_iphone())
					.max()
					.cloned()
					.map(DeviceName::from)
					.ok_or_else(|| {
						eyre!("No booted iPhones found!")
							.with_suggestion(|| {
								"try running `apple-clis xcrun simctl boot --iphone` to boot a simulator"
							})
							.with_note(|| format!("Booted devices: {:?}", &booted_devices))
					}),
				(false, false, false, Some(name)) => {
					if booted_devices.iter().any(|d| d.name == name) {
						Ok(name)
					} else {
						Err(
							eyre!("The provided device name is not booted")
								.with_suggestion(|| {
									format!(
										"try running `apple-clis xcrun simctl boot --name {}` to boot a simulator",
										name
									)
								})
								.with_note(|| format!("Booted devices: {:?}", &booted_devices)),
						)
					}
				}
				_ => Err(eyre!("Clap arguments should prevent this")),
			}
		}
	}
}
