use bossy::ExitStatus;
use clap::ValueEnum;

use crate::prelude::*;

use super::OpenCLIInstance;

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum, Debug)]
pub enum WellKnown {
	/// Opens "/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app"
	Simulator,
}

impl TryFrom<&WellKnown> for &'static Utf8Path {
	type Error = error::Error;

	#[tracing::instrument(level = "trace", skip(value))]
	/// Path may be invalid
	fn try_from(value: &WellKnown) -> std::result::Result<Self, Self::Error> {
		let path: &'static Utf8Path = match value {
			WellKnown::Simulator => {
				Utf8Path::new("/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app")
			}
		};
		match path.try_exists() {
			Ok(true) => Ok(path),
			Ok(false) => Err(Error::WellKnownPathNotFound {
				hard_coded_path: path.to_owned(),
				err: None,
			}),
			Err(err) => Err(Error::WellKnownPathNotFound {
				hard_coded_path: path.to_owned(),
				err: Some(err),
			}),
		}
	}
}

impl WellKnown {
	#[tracing::instrument(level = "trace", skip(self))]
	/// Fails if path doesn't exist
	pub fn get_path(&self) -> error::Result<&Utf8Path> {
		self.try_into()
	}
}

impl OpenCLIInstance {
	#[tracing::instrument(level = "trace", skip(self, well_known))]
	pub fn open_well_known(&self, well_known: &WellKnown) -> Result<ExitStatus> {
		let path = well_known.get_path()?;
		Ok(self.bossy_command().with_arg(path).run_and_wait()?)
	}
}
