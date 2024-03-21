use crate::prelude::*;

use super::XcRunSimctlInstance;

pub use output::*;
mod output;

impl<'src> XcRunSimctlInstance<'src> {
	#[instrument(skip_all, ret)]
	pub fn list(&self) -> Result<ListOutput> {
		ListOutput::from_bossy_result(self
			.bossy_command()
			.with_arg("list")
			.with_arg("--json")
			.run_and_wait_for_output())
	}
}
