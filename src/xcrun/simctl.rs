use crate::prelude::ExecInstance;

use super::XcRunInstance;

pub mod boot;
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

impl XcRunSimctlInstance<'_> {
	fn bossy_command(&self) -> bossy::Command {
		self.xc_run_instance.bossy_command().with_arg("simctl")
	}
}
