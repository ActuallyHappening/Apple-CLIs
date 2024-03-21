use crate::prelude::*;

use super::XcRunSimctlInstance;

pub use output::*;
mod output;

impl<'src> XcRunSimctlInstance<'src> {
	pub fn list(&self) -> Result<ListJson> {
		let output = self
			.bossy_command()
			.with_arg("list")
			.with_arg("--json")
			.run_and_wait_for_string()?;

		Ok(serde_json::from_str(&output)?)
	}
}

#[cfg(test)]
mod tests {
	use tracing::debug;

	use super::*;

	#[test]
	fn test_simctl_list() {
		let example = include_str!("../../../tests/simctl-list-full.json");
		let output = serde_json::from_str::<ListJson>(example);
		match output {
			Ok(output) => {
				debug!("Output: {:?}", output);
			}
			Err(e) => {
				panic!("Error parsing: {:?}", e)
			}
		}
	}
}
