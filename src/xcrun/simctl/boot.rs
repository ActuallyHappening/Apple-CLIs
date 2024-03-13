use crate::{prelude::*, shared::identifiers::DeviceName};

use super::XcRunSimctlInstance;

#[derive(thiserror::Error, Debug)]
pub enum BootError {}

impl XcRunSimctlInstance<'_> {
	pub fn boot(&self, device_name: DeviceName) -> Result<(), BootError> {
		let command = self
			.bossy_command()
			.with_arg("boot")
			.with_arg(device_name.to_string());

		todo!()
	}
}
