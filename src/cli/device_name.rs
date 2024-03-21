use crate::prelude::*;
use crate::xcrun::simctl::XcRunSimctlInstance;

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DeviceSimulatorUnBootedArgs {
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
#[group(required = true)]
pub struct DeviceSimulatorBootedArgs {
	/// Will find *any* device that is already booted
	/// as opposed to [DeviceSimulatorBooted].ipad (--ipad) or [DeviceSimulatorBooted].iphone (--iphone)
	#[arg(long, group = "device_name")]
	booted: bool,

	/// Will automatically boot a device if none matching the criteria is already booted
	#[arg(long, requires = "device_name")]
	auto_boot: bool,

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

impl DeviceSimulatorUnBootedArgs {
	pub fn resolve(self, simctl_instance: &XcRunSimctlInstance) -> color_eyre::Result<DeviceName> {
		match (self.ipad, self.iphone, self.name) {
			(false, false, Some(name)) => Ok(name),
			(true, false, None) => {
				let list = simctl_instance.list()?;
				let latest_ipad = list
					.get_success_reported()?
					.devices()
					.names()
					.ipads()
					.max()
					.ok_or_else(|| eyre!("No simulator iPads found!"))?;
				Ok((*latest_ipad).into())
			}
			(false, true, None) => {
				let list_output = &simctl_instance.list()?;
				let latest_iphone = list_output
					.get_success_reported()?
					.devices()
					.names()
					.iphones()
					.max()
					.ok_or_else(|| eyre!("No simulator iPhones found!"))?;
				Ok((*latest_iphone).into())
			}
			_ => Err(eyre!("Clap arguments should prevent this")),
		}
	}
}

impl DeviceSimulatorBootedArgs {
	pub fn resolve(self, simctl_instance: &XcRunSimctlInstance) -> color_eyre::Result<DeviceName> {
		let list = simctl_instance.list()?;
		let booted_devices = list
			.get_success_reported()?
			.devices()
			.filter(|d| d.ready())
			.collect::<Vec<_>>();
		if booted_devices.is_empty() && !self.auto_boot {
			Err(
				eyre!("No booted devices found!")
					.with_suggestion(|| "try using the flag --auto-boot")
					.with_suggestion(|| "try running `apple-clis xcrun simctl boot` to boot a simulator"),
			)
		} else {
			match (
				self.booted,
				self.ipad,
				self.iphone,
				self.name,
				self.auto_boot,
			) {
				(true, false, false, None, auto_boot) => {
					if auto_boot {
						let a_device_name = &list
							.get_success_reported()?
							.devices()
							.a_device()
							.ok_or_else(|| eyre!("Couldn't find any simulators installed"))?
							.name;
						info!(device_name = %a_device_name, "Since auto-boot is enabled, booting a simulator");
						simctl_instance.boot(a_device_name)?;
						Ok(a_device_name.clone())
					} else {
						Ok(booted_devices[0].name.clone())
					}
				}
				(false, true, false, None, auto_boot) => {
					if !auto_boot {
						booted_devices
							.iter()
							.filter_map(|d| d.name.get_ipad())
							.max()
							.cloned()
							.map(DeviceName::from)
							.ok_or_else(|| eyre!("No booted iPads found!"))
							.with_suggestion(|| {
								"try running `apple-clis xcrun simctl boot --ipad` to boot a simulator"
							})
							.with_note(|| format!("Booted devices: {:?}", &booted_devices))
					} else {
						let a_device_name = (*list
							.get_success_reported()?
							.devices()
							.names()
							.an_ipad()
							.ok_or_else(|| eyre!("Couldn't find any iPad simulators installed"))?)
						.into();
						info!(device_name = %a_device_name, "Since auto-boot is enabled, booting an iPad simulator");
						simctl_instance.boot(&a_device_name)?;
						Ok(a_device_name)
					}
				}
				(false, false, true, None, auto_boot) => {
					if !auto_boot {
						booted_devices
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
							})
					} else {
						let a_device_name: DeviceName = (*list
							.get_success_reported()?
							.devices()
							.names()
							.an_iphone()
							.ok_or_else(|| eyre!("Couldn't find any iPhone simulators installed"))?)
						.into();
						info!(device_name = %a_device_name, "Since auto-boot is enabled, booting an iPhone simulator");
						simctl_instance.boot(&a_device_name)?;
						Ok(a_device_name)
					}
				}
				(false, false, false, Some(name), auto_boot) => {
					if !list
						.get_success_reported()?
						.devices()
						.map(|d| &d.name)
						.any(|n| n == &name)
					{
						Err(
							eyre!("The provided device name is not installed")
								.with_suggestion(|| {
									format!(
										"try running `apple-clis xcrun simctl boot --name {}` to boot a simulator",
										name
									)
								})
								.with_suggestion(|| {
									"try running `apple-clis xcrun simctl list` to see installed simulators"
								})
								.with_note(|| {
									format!(
										"Installed devices: {:?}",
										&list.unwrap_success().devices().collect::<Vec<_>>()
									)
								}),
						)
					} else if auto_boot {
						simctl_instance.boot(&name)?;
						Ok(name)
					} else {
						Ok(name)
					}
				}
				_ => Err(eyre!("Clap arguments should prevent this")),
			}
		}
	}
}
