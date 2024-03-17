use crate::{prelude::*, shared::identifiers::DeviceName};

pub use self::boot_output::BootOutput;

use super::XcRunSimctlInstance;

mod boot_output;

impl XcRunSimctlInstance<'_> {
	/// This will not fail if the device is already booted,
	/// but will return [BootOutput::AlreadyBooted] in that case.
	#[tracing::instrument(level = "trace", skip(self, device_name))]
	#[must_use = "This operation may have failed, check `BootOutput.success()`"]
	pub fn boot(&self, device_name: impl Into<DeviceName>) -> Result<BootOutput> {
		BootOutput::from_output(
			self
				.bossy_command()
				.with_arg("boot")
				.with_arg(device_name.into().to_string())
				.run_and_wait_for_output(),
		)
	}
}
