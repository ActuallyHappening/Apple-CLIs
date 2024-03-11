use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub mod ios_deploy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	device_identifier: String,

	device_name: String,

	model_name: String,

	interface: String,
}
