use serde::{Deserialize, Serialize};

pub mod ios_deploy;
pub mod security;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub device_identifier: String,
	pub device_name: String,
	pub model_name: String,
	pub interface: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateInstanceError {
	#[error("Error running `which ios-deploy`: {0}")]
	CommandExecution(#[from] which::Error),

	#[error("Error converting path to UTF-8: {0}")]
	PathNotUtf8(#[from] camino::FromPathBufError),
}
