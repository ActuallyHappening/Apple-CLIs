use serde::Deserialize;
use tracing::debug;

use crate::Device;

use super::IosDeployInstance;

#[derive(thiserror::Error, Debug)]
pub enum IosDeployDetectError {
	#[error("Failed to parse `ios-deploy --detect --json` output: {0}")]
	ParseError(#[from] serde_json::Error),

	#[error("An error occured while executing `ios-deploy --detect`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

impl IosDeployInstance {
	pub fn detect_devices(&self) -> Result<Vec<Device>, IosDeployDetectError> {
		let output = self
			.bossy_command()
			.with_arg("--detect")
			.with_arg("--json")
			.run_and_wait_for_string()?;

		#[derive(Debug, Deserialize)]
		struct Event {
			#[serde(rename(deserialize = "Interface"))]
			interface: String,

			#[serde(rename(deserialize = "Device"))]
			device: DeviceDetected,
		}

		#[derive(Debug, Deserialize)]
		struct DeviceDetected {
			#[serde(rename(deserialize = "DeviceIdentifier"))]
			device_identifier: String,

			#[serde(rename(deserialize = "DeviceName"))]
			device_name: String,

			#[serde(rename(deserialize = "modelName"))]
			model_name: String,
		}

		let events = serde_json::from_str::<Vec<Event>>(&output)?;
		let devices = events
			.into_iter()
			.map(|event| Device {
				device_name: event.device.device_name,
				device_identifier: event.device.device_identifier,
				model_name: event.device.model_name,
				interface: event.interface,
			})
			.collect();

		Ok(devices)
	}
}
