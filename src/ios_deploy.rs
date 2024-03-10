use anyhow::Context;
use clap::Args;

use crate::{Device, RustProjectInfo};

/// Deploy an iOS app bundle to a real device
#[derive(Args, Debug)]
pub struct IosDeploy {
    #[arg(long)]
    debug: bool,

    /// The ID of the device you are connected to, e.g.
    /// 00008693-0517604C71E3421F.
    ///
    /// Will try to infer the device ID from the environment if not provided.
    device: Option<String>,
}

impl IosDeploy {
    pub fn ios_deploy(&self, project_info: &RustProjectInfo) -> anyhow::Result<()> {
				let mut command = bossy::Command::pure("ios-deploy");

				if self.debug {
					command.add_arg("--debug");
				}

				command.run_and_wait().context("ios-deploy failed")?;
						
        Ok(())
    }
}
