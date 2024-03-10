use anyhow::Context;
use tracing::debug;

use crate::Device;

pub fn list_real() -> anyhow::Result<impl Iterator<Item = Device>> {
    let output = bossy::Command::pure("ios-deploy")
        .with_arg("--detect")
        .run_and_wait_for_string()
        .context("ios-deploy --detect failed")?;

    for line in output.lines() {
        if line.starts_with("[....] Found ") {
            let device = line.split("Found ").last().context("Failed to parse")?;
            debug!("Found device: {}", device);
        }
    }

    todo!()
}
