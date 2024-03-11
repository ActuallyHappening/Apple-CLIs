use serde::Serialize;

pub mod ios_deploy;

#[derive(Debug, Serialize)]
pub struct Device {
	id: String,
}

impl Device {
	pub fn new(id: impl Into<String>) -> Self {
		Device { id: id.into() }
	}
}
