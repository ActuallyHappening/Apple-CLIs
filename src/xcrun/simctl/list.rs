use crate::prelude::*;
use crate::shared::identifiers::device_name::IPadVariant;
use crate::shared::{
	identifiers::{
		device_name::{DeviceName, IPhoneVariant},
		RuntimeIdentifier,
	},
	Device,
};

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
	devices: HashMap<RuntimeIdentifier, Vec<ListDevice>>,
}

impl ListOutput {
	pub fn devices(&self) -> impl Iterator<Item = &ListDevice> {
		self.devices.values().flatten()
	}

	pub fn iphones(&self) -> impl Iterator<Item = &IPhoneVariant> {
		self.devices().filter_map(|device| match device.name {
			DeviceName::IPhone(ref variant) => Some(variant),
			_ => None,
		})
	}

	pub fn ipads(&self) -> impl Iterator<Item = &IPadVariant> {
		self.devices().filter_map(|device| match device.name {
			DeviceName::IPad(ref variant) => Some(variant),
			_ => None,
		})
	}
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListDevice {
	pub availability_error: Option<String>,
	pub data_path: Utf8PathBuf,
	pub log_path: Utf8PathBuf,
	pub udid: String,
	pub is_available: bool,
	pub device_type_identifier: String,
	pub state: State,

	pub name: DeviceName,
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
	pub fn list(&self) -> Result<ListOutput, ListError> {
		let output = self
			.bossy_command()
			.with_arg("list")
			.with_arg("--json")
			.run_and_wait_for_string()?;

		Ok(serde_json::from_str(&output)?)
	}
}

#[cfg(test)]
mod tests {
	use tracing::debug;

	use super::*;

	#[test]
	fn test_simctl_list() {
		let example = include_str!("../../../tests/simctl-list-full.json");
		let output = serde_json::from_str::<ListOutput>(example);
		match output {
			Ok(output) => {
				debug!("Output: {:?}", output);
			}
			Err(e) => {
				panic!("Error parsing: {:?}", e)
			}
		}
	}
}
