use crate::Device;
use serde::Deserialize;

use super::IosDeployInstance;

#[derive(thiserror::Error, Debug)]
pub enum IosDeployDetectError {
	#[error("Failed to parse `ios-deploy --detect --json` output: {0}")]
	ParseError(#[from] serde_json::Error),

	#[error("An error occurred while executing `ios-deploy --detect`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

#[derive(Debug)]
pub struct DetectDevicesConfig {
	pub timeout: u8,
	pub wifi: bool,
}

impl Default for DetectDevicesConfig {
	fn default() -> Self {
		DetectDevicesConfig {
			timeout: 1,
			wifi: true,
		}
	}
}

impl IosDeployInstance {
	/// Uses default [DetectDevicesConfig].
	pub fn detect_devices(&self) -> Result<Vec<Device>, IosDeployDetectError> {
		self.detect_devices_with_config(&DetectDevicesConfig::default())
	}

	pub fn detect_devices_with_config(
		&self,
		config: &DetectDevicesConfig,
	) -> Result<Vec<Device>, IosDeployDetectError> {
		let mut command = self
			.bossy_command()
			.with_arg("--detect")
			.with_arg("--json")
			.with_args(["--timeout", config.timeout.to_string().as_str()]);

		if !config.wifi {
			command.add_arg("--wifi");
		}

		let output = command.run_and_wait_for_string()?;
		// wraps in [] so that bunch of json objects can be deserialized
		let output = format!("[{}]", output);

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
