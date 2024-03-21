use crate::prelude::*;
use crate::shared::identifiers::IPadVariant;
use crate::shared::identifiers::{DeviceName, IPhoneVariant, RuntimeIdentifier};

#[derive(Debug, Serialize)]
#[non_exhaustive]
pub enum ListOutput {
	SuccessJson(ListJson),

	SuccessUnImplemented { stdout: String },

	ErrorUnImplemented { stderr: String },
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

impl ListJson {
	pub fn devices(&self) -> impl Iterator<Item = &ListDevice> + '_ {
		self.devices.values().flatten()
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
