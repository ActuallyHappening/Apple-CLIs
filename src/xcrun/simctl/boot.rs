use bossy::ExitStatus;

use crate::{prelude::*, shared::identifiers::DeviceName};

pub use self::boot_output::BootOutput;

use super::XcRunSimctlInstance;

mod boot_output;

impl XcRunSimctlInstance<'_> {
	#[tracing::instrument(level = "trace", skip(self, device_name))]
	pub fn boot(&self, device_name: DeviceName) -> Result<BootOutput> {
		BootOutput::from_output(
			self
				.bossy_command()
				.with_arg("boot")
				.with_arg(device_name.to_string())
				.run_and_wait_for_output(),
		)
	}
}
