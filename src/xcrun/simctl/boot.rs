use crate::{prelude::*, shared::identifiers::DeviceName};

use super::XcRunSimctlInstance;

#[derive(thiserror::Error, Debug)]
pub enum BootError {
	#[error("Failed to execute `xcrun simctl boot`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

impl XcRunSimctlInstance<'_> {
	pub fn boot(&self, device_name: DeviceName) -> Result<(), BootError> {
		self
			.bossy_command()
			.with_arg("boot")
			.with_arg(device_name.to_string()).run_and_wait()?;
		Ok(())
	}
}
