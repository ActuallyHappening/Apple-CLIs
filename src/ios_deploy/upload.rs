use crate::prelude::*;

use super::{detect::Device, IosDeployCLIInstance};

pub use output::*;
mod output;

impl IosDeployCLIInstance {
	#[instrument(ret, skip_all)]
	pub fn upload_bundle(
		&self,
		device: &Device,
		bundle_path: impl AsRef<Utf8Path>,
	) -> Result<UploadOutput> {
		let mut command = self
			.bossy_command()
			.with_args(["--id", &device.device_identifier])
			.with_args(["--bundle", bundle_path.as_ref().as_str()]);

		// if self.debug {
		command.add_arg("--debug");
		// }

		UploadOutput::from_bossy_result(command.run_and_wait_for_output())
	}
}
