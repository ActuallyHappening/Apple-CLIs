use crate::prelude::*;

use bossy::ExitStatus;
use camino::Utf8Path;

use crate::shared::Device;

use super::IosDeployCLIInstance;

impl IosDeployCLIInstance {
	#[tracing::instrument(level = "trace", skip(self, device, bundle_path))]
	pub fn upload_bundle(
		&self,
		device: &Device,
		bundle_path: impl AsRef<Utf8Path>,
	) -> Result<ExitStatus> {
		let mut command = self
			.bossy_command()
			.with_args(["--id", &device.device_identifier])
			.with_args(["--bundle", bundle_path.as_ref().as_str()]);

		// if self.debug {
		command.add_arg("--debug");
		// }

		Ok(command.run_and_wait()?)
	}
}
