use std::num::NonZeroU8;

use serde::{de::DeserializeOwned, Deserialize};

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);

pub struct DeviceName {
	pub device_type: BaseDeviceType,
}

pub enum BaseDeviceType {
	IPhone,
}

#[derive(Debug)]
pub enum IPhoneVariant {
	SE,
	Num(NonZeroU8),
}

impl DeserializeOwned

#[cfg(test)]
mod tests {

	$[test]
	fn parse_device_name() {
		let examples = ["iPhone 14 Plus", "iPhone SE (3rd generation)"];
	}

	// #[test]
	// fn parse_runtime_identifier() {
	// 	let input = r#""com.apple.CoreSimulator.SimRuntime.iOS-16-4""#;
	// 	let output: super::RuntimeIdentifier = serde_json::from_str(input).unwrap();
	// 	assert_eq!(output.0, "com.apple.CoreSimulator.SimRuntime.iOS-16-4");
	// }
}