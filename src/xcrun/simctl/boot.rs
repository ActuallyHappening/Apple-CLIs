use crate::{prelude::*, shared::identifiers::DeviceName};

use super::XcRunSimctlInstance;

pub use self::output::BootOutput;
mod output;

impl XcRunSimctlInstance<'_> {
	/// This will not fail if the device is already booted,
	/// but will return [BootOutput::AlreadyBooted] in that case.
	#[tracing::instrument(skip_all, ret)]
	pub fn boot(&self, device_name: impl Into<DeviceName>) -> Result<BootOutput> {
		BootOutput::from_bossy_result(
			self
				.bossy_command()
				.with_arg("boot")
				.with_arg(device_name.into().to_string())
				.run_and_wait_for_output(),
		)
	}
}
