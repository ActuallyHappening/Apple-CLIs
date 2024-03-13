use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

pub mod identifiers;

pub mod prelude {
	pub use super::identifiers::*;
	pub(super) use super::{ws, NomFromStr};
	pub(crate) use super::{CreateInstanceError, ExecInstance};
	pub(super) use crate::prelude::*;
	#[allow(unused_imports)]
	pub(super) use nom::{
		branch::alt,
		bytes::complete::{tag, take_till, take_until},
		character::complete::{alpha0, alpha1, digit1, space0, space1},
		combinator::{map, map_res, peek, rest, success, value},
		number::complete::float,
		sequence::tuple,
		sequence::{delimited, preceded, terminated},
		IResult,
	};
	pub(super) use strum::EnumDiscriminants;
}
use prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
	pub device_identifier: String,
	pub device_name: String,
	pub model_name: String,
	pub interface: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateInstanceError {
	#[error("Error running `which ios-deploy`: {0}")]
	CommandExecution(#[from] which::Error),

	#[error("Error converting path to UTF-8: {0}")]
	PathNotUtf8(#[from] camino::FromPathBufError),

	#[error("Calling `--version` failed: {0}")]
	VersionCheckFailed(#[from] bossy::Error),

	#[error("Path does not exist: {path} (std::io::Error: {err:?})")]
	PathDoesNotExist {
		path: Utf8PathBuf,
		err: Option<std::io::Error>,
	},
}

/// Wrapper of binary
pub trait ExecInstance: Sized {
	/// E.g. `codesign` or `xcrun`
	const BINARY_NAME: &'static str;

	/// # Safety
	/// Must point to a valid executable file.
	///
	/// Prefer [ExecInstance::new]
	unsafe fn new_unchecked(exec_path: impl AsRef<Utf8Path>) -> Self;

	fn get_inner_exec_path(&self) -> &Utf8Path;

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(self.get_inner_exec_path())
	}

	fn version_command(&self) -> bossy::Command {
		self.bossy_command().with_arg("--version")
	}

	fn from_path(path: impl AsRef<Utf8Path>) -> Result<Self, CreateInstanceError> {
		// check path exists
		let path = path.as_ref();
		match path.try_exists() {
			Ok(true) => Ok(unsafe { Self::new_unchecked(path) }),
			Ok(false) => Err(CreateInstanceError::PathDoesNotExist {
				path: path.to_owned(),
				err: None,
			}),
			Err(e) => Err(CreateInstanceError::PathDoesNotExist {
				path: path.to_owned(),
				err: Some(e),
			}),
		}
	}

	/// Uses `which` to find the binary automatically
	fn new() -> Result<Self, CreateInstanceError> {
		let path = which::which(Self::BINARY_NAME)?;
		let path = Utf8PathBuf::try_from(path)?;
		let instance = unsafe { Self::new_unchecked(path) };
		Ok(instance)
	}
}

#[macro_export]
macro_rules! impl_exec_instance {
	($t:ty) => {
		impl $crate::shared::ExecInstance for $t {
			const BINARY_NAME: &'static str = "ios-deploy";

			unsafe fn new_unchecked(exec_path: impl AsRef<::camino::Utf8Path>) -> Self {
				Self {
					exec_path: exec_path.as_ref().to_path_buf(),
				}
			}

			fn get_inner_exec_path(&self) -> &::camino::Utf8Path {
				&self.exec_path
			}
		}

		impl $t {
			/// Constructs an instance of `Self` using `which`.
			pub fn new() -> Result<Self, $crate::shared::CreateInstanceError> {
				$crate::shared::ExecInstance::new()
			}
		}
	};
}

trait NomFromStr: Sized {
	fn nom_from_str(input: &str) -> IResult<&str, Self>;
}

impl NomFromStr for NonZeroU8 {
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		map_res(digit1, |s: &str| s.parse())(input)
	}
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(space0, inner, space0)
}
