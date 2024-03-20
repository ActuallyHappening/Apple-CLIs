use crate::prelude::*;

use super::XcRunSimctlInstance;

pub use output::*;
mod output {
	use crate::prelude::*;
	use crate::shared::identifiers::IPadVariant;
	use crate::shared::identifiers::{DeviceName, IPhoneVariant, RuntimeIdentifier};

	#[derive(Debug, Serialize)]
	#[non_exhaustive]
	pub enum ListOutput {
		SuccessUnImplemented {
			stdout: String,
		},

		ErrorUnImplemented {
			stderr: String,
		},
	}

	#[derive(Deserialize, Debug)]
	pub struct ListJson {
		devices: HashMap<RuntimeIdentifier, Vec<ListDevice>>,
	}

	impl ListJson {
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

		pub fn a_device(&self) -> Option<&ListDevice> {
			self.devices().next()
		}

		/// Tries to find the latest iPad in the list of devices
		/// Not necessarily booted already
		pub fn an_ipad(&self) -> Option<&IPadVariant> {
			self.ipads().max()
		}

		/// Tries to find the latest iPhone in the list of devices
		/// Not necessarily booted already
		pub fn an_iphone(&self) -> Option<&IPhoneVariant> {
			self.iphones().max()
		}
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all(deserialize = "camelCase"))]
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

	#[derive(Deserialize, Serialize, Debug)]
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
}

impl<'src> XcRunSimctlInstance<'src> {
	pub fn list(&self) -> Result<ListJson> {
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
		let output = serde_json::from_str::<ListJson>(example);
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
