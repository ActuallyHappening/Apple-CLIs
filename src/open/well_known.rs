use bossy::ExitStatus;
use clap::ValueEnum;

use crate::prelude::*;

use super::{OpenCLIInstance, OpenError};

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Debug)]
pub enum WellKnown {
	/// Opens "/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app"
	Simulator,
}

impl TryFrom<&WellKnown> for &'static Utf8Path {
	type Error = OpenError;

	/// Path may be invalid
	fn try_from(value: &WellKnown) -> Result<Self, Self::Error> {
		let path: &'static Utf8Path = match value {
			WellKnown::Simulator => {
				Utf8Path::new("/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app")
			}
		};
		match path.try_exists() {
			Ok(true) => Ok(path),
			Ok(false) => Err(OpenError::WellKnownPathNotFound {
				hard_coded_path: path.to_owned(),
				err: None,
			}),
			Err(err) => Err(OpenError::WellKnownPathNotFound {
				hard_coded_path: path.to_owned(),
				err: Some(err),
			}),
		}
	}
}

impl WellKnown {
	/// Fails if path doesn't exist
	pub fn get_path(&self) -> Result<&Utf8Path, OpenError> {
		self.try_into()
	}
}

impl OpenCLIInstance {
	pub fn open_well_known(&self, well_known: WellKnown) -> Result<ExitStatus, OpenError> {
		let path = well_known.get_path()?;
		Ok(self.bossy_command().with_arg(path).run_and_wait()?)
	}
}
