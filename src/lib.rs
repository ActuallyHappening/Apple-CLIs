use serde::{Deserialize, Serialize};

pub mod ios_deploy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub device_identifier: String,
	pub device_name: String,
	pub model_name: String,
	pub interface: String,
}
