use crate::prelude::*;

use super::XcRunInstance;

pub mod boot;
pub mod install;
pub mod list;

pub struct XcRunSimctlInstance<'src> {
	exec_parent: &'src XcRunInstance,
}

impl XcRunInstance {
	pub fn simctl(&self) -> XcRunSimctlInstance {
		XcRunSimctlInstance { exec_parent: self }
	}
}

impl XcRunSimctlInstance<'_> {
	fn bossy_command(&self) -> bossy::Command {
		self.exec_parent.bossy_command().with_arg("simctl")
	}
}

impl_exec_child!(
	XcRunSimctlInstance<'src>,
	parent = XcRunInstance,
	subcommand = "simctl"
);
