use std::collections::HashMap;

use serde::Deserialize;

use crate::shared::{identifiers::RuntimeIdentifier, Device};

use super::XcRunSimctlInstance;

#[derive(thiserror::Error, Debug)]
pub enum ListError {
	#[error("Error running `xcrun simctl list --json`: {0}")]
	CommandExecution(#[from] bossy::Error),

	#[error("Error parsing JSON output: {0}")]
	Json(#[from] serde_json::Error),
}

#[derive(Deserialize, Debug)]
pub struct ListOutput {
	devices: HashMap<RuntimeIdentifier, ListDevice>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListDevice {
	pub availability_error: Option<String>,
	pub udid: String,
	pub is_available: bool,
	pub device_type_identifier: String,
	pub state: State,
	pub name: String,
}

#[derive(Deserialize, Debug)]
pub enum State {
	Shutdown,
	Booted,
}

impl State {
	pub fn ready(&self) -> bool {
		matches!(self, State::Booted)
	}
}

impl ListDevice {
	pub fn ready(&self) -> bool {
		self.state.ready() && self.is_available
	}
}

impl<'src> XcRunSimctlInstance<'src> {
	pub fn list(&self) -> Result<Vec<Device>, ListError> {
		let output = self
			.bossy_command()
			.with_arg("list")
			.with_arg("--json")
			.run_and_wait_for_string()?;

		let data: ListOutput = serde_json::from_str(&output)?;

		println!("Data: {:?}", data);

		todo!();
	}
}
