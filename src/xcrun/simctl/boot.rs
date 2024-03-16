use bossy::ExitStatus;

use crate::{prelude::*, shared::identifiers::DeviceName};

use super::XcRunSimctlInstance;

impl XcRunSimctlInstance<'_> {
	#[tracing::instrument(level = "trace", skip(self, device_name))]
	pub fn boot(&self, device_name: DeviceName) -> Result<ExitStatus> {
		Ok(
			self
				.bossy_command()
				.with_arg("boot")
				.with_arg(device_name.to_string())
				.run_and_wait()?,
		)
	}
}
