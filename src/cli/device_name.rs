use color_eyre::eyre::eyre;

use crate::{prelude::DeviceName, xcrun::simctl::XcRunSimctlInstance};

#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct DeviceSimulator {
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

impl DeviceSimulator {
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
				Ok(latest_ipad.clone().into())
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
