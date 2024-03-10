use tracing::debug;

use crate::Device;

#[derive(thiserror::Error, Debug)]
pub enum IosDeployDetectError {
    #[error("Failed to parse `ios-deploy --detect` output, line {line_num}: {line}")]
    ParseError { line_num: usize, line: String },

    #[error("An error occured while executing `ios-deploy --detect`: {0}")]
    ExecuteError(#[from] bossy::Error),
}

pub fn list_real() -> Result<Vec<Device>, IosDeployDetectError> {
    let output = bossy::Command::pure("/opt/homebrew/bin/ios-deploy")
        .with_arg("--detect")
        .run_and_wait_for_string()?;

    let mut devices = Vec::new();
    for (line_num, line) in output.lines().enumerate() {
        if line.starts_with("[....] Found ") {
            let device = line
                .split("Found ")
                .last()
                .ok_or(IosDeployDetectError::ParseError {
                    line_num,
                    line: line.into(),
                })?;
            debug!("Found device raw: {}", device);
            let id = device
                .split_whitespace()
                .next()
                .ok_or(IosDeployDetectError::ParseError {
                    line_num,
                    line: line.into(),
                })?
                .to_string();
            let device = Device::new(id);
						debug!("Found device: {:?}", device);
            devices.push(device);
        }
    }

    Ok(devices)
}
