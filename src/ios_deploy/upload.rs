use camino::Utf8Path;

use crate::Device;

use super::IosDeployCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum UploadBundleError {
	#[error("No real devices were found. Try running `ios-deploy --detect`.")]
	NoDevicesFound,

	#[error("An error occured while executing `ios-deploy`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

impl IosDeployCLIInstance {
	pub fn upload_bundle(
		&self,
		device: &Device,
		bundle_path: impl AsRef<Utf8Path>,
	) -> Result<(), UploadBundleError> {
		let mut command = self
			.bossy_command()
			.with_args(["--id", &device.device_identifier])
			.with_args(["--bundle", bundle_path.as_ref().as_str()]);

		// if self.debug {
		command.add_arg("--debug");
		// }

		command.run_and_wait()?;

		Ok(())
	}
}
