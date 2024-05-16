use crate::prelude::*;
use crate::shared::identifiers::IPadVariant;
use crate::shared::identifiers::{DeviceName, IPhoneVariant, RuntimeIdentifier};

#[derive(Debug, Serialize)]
#[non_exhaustive]
#[must_use = include_doc!(must_use_cmd_output)]
pub enum ListOutput {
	/// Contains the actual output of the command, parsed
	SuccessJson(ListJson),

	#[doc = include_doc!(cmd_success)]
	SuccessUnImplemented { stdout: String },

	#[doc = include_doc!(cmd_error)]
	ErrorUnImplemented { stderr: String },
}

impl ListOutput {
	/// Returns [Error::OutputErrored] if it didn't succeed.
	/// Used to make error handling of non-successful commands explicit
	pub fn success(&self) -> Result<&ListJson> {
		match self {
			ListOutput::SuccessJson(output) => Ok(output),
			_ => Err(Error::output_errored(self)),
		}
	}

	pub fn unwrap_success(&self) -> &ListJson {
		match self {
			ListOutput::SuccessJson(output) => output,
			_ => {
				error!(?self, "Tried to unwrap a non-successful ListOutput");
				panic!("Tried to unwrap a non-successful ListOutput")
			}
		}
	}

	pub fn get_success(&self) -> Option<&ListJson> {
		match self {
			ListOutput::SuccessJson(output) => Some(output),
			_ => None,
		}
	}

	/// Only used in CLI
	/// prefer [Self::get_success]
	#[cfg(feature = "cli")]
	pub fn get_success_reported(&self) -> std::result::Result<&ListJson, color_eyre::Report> {
		match self {
			Self::SuccessJson(output) => Ok(output),
			Self::ErrorUnImplemented { stderr } => Err(eyre!(
				"xcrun simctl list output didn't exist successfully: {:?}",
				stderr
			)),
			Self::SuccessUnImplemented { stdout } => Err(eyre!(
				"xcrun simctl list output didn't produce valid output: {:?}",
				stdout
			)),
		}
	}
}

impl CommandNomParsable for ListOutput {
	fn success_unimplemented(stdout: String) -> Self {
		Self::SuccessUnImplemented { stdout }
	}

	fn error_unimplemented(stderr: String) -> Self {
		Self::ErrorUnImplemented { stderr }
	}

	fn success_from_str(input: &str) -> Self {
		match serde_json::from_str(input) {
			Ok(output) => Self::SuccessJson(output),
			Err(e) => {
				error!(?e, "Failed to parse JSON");
				Self::success_unimplemented(input.to_owned())
			}
		}
	}
}

impl PublicCommandOutput for ListOutput {
	type PrimarySuccess = ListJson;

	fn success(&self) -> Result<&Self::PrimarySuccess> {
		match self {
			ListOutput::SuccessJson(output) => Ok(output),
			_ => Err(Error::output_errored(self)),
		}
	}
}

impl ListJson {
	/// Returns an iterator over the returned devices
	pub fn devices(&self) -> impl Iterator<Item = &ListDevice> + '_ {
		self.devices.values().flatten()
	}
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListJson {
	devices: HashMap<RuntimeIdentifier, Vec<ListDevice>>,
}

/// Allows for easier extraction of semantic information from
/// an iterator over [ListDevice]s.
#[extension_traits::extension(pub trait ListDevicesExt)]
impl<'src, T> T
where
	T: Iterator<Item = &'src ListDevice>,
{
	/// Consumes self,
	fn names(self) -> impl Iterator<Item = &'src DeviceName> {
		self.map(|device| &device.name)
	}

	fn a_device(mut self) -> Option<&'src ListDevice> {
		self.next()
	}
}

#[extension_traits::extension(pub trait ListDeviceNamesExt)]
impl<'src, T> T
where
	T: Iterator<Item = &'src DeviceName>,
{
	/// Tries to find the latest iPad in the list of devices
	/// Not necessarily booted already
	fn an_ipad(self) -> Option<&'src IPadVariant> {
		self.ipads().max()
	}

	/// Tries to find the latest iPhone in the list of devices
	/// Not necessarily booted already
	fn an_iphone(self) -> Option<&'src IPhoneVariant> {
		self.iphones().max()
	}

	fn iphones(self) -> impl Iterator<Item = &'src IPhoneVariant> {
		self.filter_map(|names| match names {
			DeviceName::IPhone(ref variant) => Some(variant),
			_ => None,
		})
	}

	fn ipads(self) -> impl Iterator<Item = &'src IPadVariant> {
		self.filter_map(|names| match names {
			DeviceName::IPad(ref variant) => Some(variant),
			_ => None,
		})
	}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
	use tracing::debug;

	use super::*;

	#[test]
	fn test_simctl_list() {
		let example = include_str!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/tests/simctl-list-full.json"
		));
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
