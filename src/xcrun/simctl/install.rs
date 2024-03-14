use bossy::ExitStatus;

use super::XcRunSimctlInstance;
use crate::prelude::*;

impl XcRunSimctlInstance<'_> {
	pub fn install_booted(&self, app_path: impl AsRef<Utf8Path>) -> Result<ExitStatus> {
		let app_path = app_path.as_ref();
		Ok(
			self
				.bossy_command()
				.with_arg("install")
				.with_arg("booted")
				.with_arg(app_path)
				.run_and_wait()?,
		)
	}
}
