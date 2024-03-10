use camino::Utf8Path;
use clap::Args;
use tracing::{info, warn};

use crate::{
	list_real::{list_real, IosDeployDetectError},
	Device,
};

#[derive(thiserror::Error, Debug)]
pub enum IosDeployBundleError {
	#[error("No device ID was passed, and an error occurred trying to list the real devices: {0}")]
	ErrorInferringDevices(#[from] IosDeployDetectError),

	#[error("No real devices were found. Try running `ios-deploy --detect`.")]
	NoDevicesFound,

	#[error("An error occured while executing `ios-deploy`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

/// Deploy an iOS app bundle to a real device
#[derive(Args, Debug)]
pub struct IosDeploy {
	#[arg(long)]
	debug: bool,

	/// The ID of the device you are connected to, e.g.
	/// 00008693-0517604C71E3421F.
	///
	/// Will try to infer the device ID from the environment if not provided.
	device: Option<String>,
}

impl IosDeploy {
	fn infer_device(&self) -> Result<Device, IosDeployBundleError> {
		match &self.device {
			Some(id) => Ok(Device::new(id)),
			None => {
				let devices = list_real()?;
				let len = devices.len();
				match devices.into_iter().next() {
					None => Err(IosDeployBundleError::NoDevicesFound),
					Some(device) => {
						if len > 1 {
							warn!(
								"More than one device found. Using the first one: {:?}",
								device
							);
						} else {
							info!(
								"Since no device ID was passed, using found device: {:?}",
								device
							);
						}
						Ok(device)
					}
				}
			}
		}
	}

	pub fn execute(&self, bundle_path: &Utf8Path) -> Result<(), IosDeployBundleError> {
		let device = self.infer_device()?;
		let mut command = bossy::Command::pure("ios-deploy")
			.with_args(["--id", &device.id])
			.with_args(["--bundle", bundle_path.as_str()]);

		if self.debug {
			command.add_arg("--debug");
		}

		command.run_and_wait()?;

		Ok(())
	}
}
