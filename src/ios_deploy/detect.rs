use super::IosDeployCLIInstance;
use crate::prelude::*;

impl IosDeployCLIInstance {
	#[instrument(ret, skip_all)]
	pub fn detect_devices(&self, config: &DetectDevicesConfig) -> Result<Vec<Device>> {
		let mut command = self
			.bossy_command()
			.with_arg("--detect")
			.with_arg("--json")
			.with_args(["--timeout", config.timeout.to_string().as_str()]);

		if !config.wifi {
			command.add_arg("--no-wifi");
		}

		let output = match command.run_and_wait_for_string() {
			Ok(output) => output,
			Err(err) => {
				if err.status().and_then(|s| s.code()) == Some(253) {
					info!(exit_status = ?err.status(), "No devices detected, since ios-deploy exited with status code 253");
					return Ok(vec![]);
				}
				Err(err)?
			}
		};

		trace!(previous_output = %output, "Before processing ios-deploy JSON output");

		// after every } close brace, adds a comma
		// this is to handle { .. } \n { ... } even style messages
		let output = format!("[{}]", output);
		let output = output.replace("}{", "},{");

		trace!(after_output = %output, "After processing ios-deploy JSON output");

		#[derive(Debug, Deserialize)]
		struct Event {
			#[serde(rename(deserialize = "Interface"))]
			interface: DeviceInterface,

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
			model_name: ModelName,
		}

		let events = match serde_json::from_str::<Vec<Event>>(&output) {
			Ok(events) => events,
			Err(err) => {
				error!(%output, "Failed to parse JSON output from ios-deploy");
				Err(err)?
			}
		};
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

#[derive(Debug)]
#[cfg_attr(feature = "cli", derive(clap::Args))]
pub struct DetectDevicesConfig {
	#[cfg_attr(feature = "cli", clap(long, default_value_t = 1))]
	pub timeout: u8,

	#[cfg_attr(feature = "cli", clap(long, default_value_t = false))]
	pub wifi: bool,
}

impl Default for DetectDevicesConfig {
	#[instrument(level = "trace", skip())]
	fn default() -> Self {
		DetectDevicesConfig {
			timeout: 1,
			wifi: true,
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub device_identifier: String,
	pub device_name: String,
	pub model_name: ModelName,
	pub interface: DeviceInterface,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceInterface {
	Usb,
	Wifi,
}