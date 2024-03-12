use crate::shared::{Device, ExecCommand};

use super::XcRunInstance;

pub mod list;

pub struct XcRunSimctlInstance<'src> {
	xc_run_instance: &'src XcRunInstance,
}

impl XcRunInstance {
	pub fn simctl(&self) -> XcRunSimctlInstance {
		XcRunSimctlInstance {
			xc_run_instance: self,
		}
	}
}

impl ExecCommand for XcRunSimctlInstance<'_> {
	const COMMAND_SUFFIX: &'static str = "simctl";

	fn bossy_command(&self) -> bossy::Command {
		self
			.xc_run_instance
			.bossy_command()
			.with_arg(Self::COMMAND_SUFFIX)
	}
}
