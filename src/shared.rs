use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

pub mod identifiers;

pub mod prelude {
	pub use super::identifiers::*;
	pub(crate) use super::ExecInstance;
	pub(super) use super::{ws, NomFromStr};
	pub(crate) use crate::impl_exec_instance;
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

	fn validate_version(&self) -> bool {
		self
			.version_command()
			.run_and_wait()
			.is_ok_and(|status| status.success())
	}

	fn from_path(path: impl AsRef<Utf8Path>) -> Result<Self> {
		// check path exists
		let path = path.as_ref();
		match path.try_exists() {
			Ok(true) => Ok(unsafe { Self::new_unchecked(path) }),
			Ok(false) => Err(Error::PathDoesNotExist {
				path: path.to_owned(),
				err: None,
			}),
			Err(e) => Err(Error::PathDoesNotExist {
				path: path.to_owned(),
				err: Some(e),
			}),
		}
	}

	/// Uses `which` to find the binary automatically
	fn new() -> Result<Self> {
		let path = which::which(Self::BINARY_NAME)?;
		let path = Utf8PathBuf::try_from(path)?;
		// Safety: `path` is a valid path to the binary
		let instance = unsafe { Self::new_unchecked(path) };
		if !instance.validate_version() {
			Err(Error::VersionCheckFailed)
		} else {
			Ok(instance)
		}
	}
}

pub trait ExecChild<'src>: Sized {
	const SUBCOMMAND_NAME: &'static str;

	type Parent: ExecInstance;

	/// Unsafe constructor for a child command.
	/// Assumes that the subcommand has been validated to exist.
	///
	/// # Safety
	/// Parent must be validated. Type system should validate this,
	/// this method being unsafe is to reflect [ExecInstance::new_unchecked].
	///
	/// Prefer [ChildExec::new]
	unsafe fn new_unchecked(parent: &'src Self::Parent) -> Self;
	fn get_inner_parent(&self) -> &Self::Parent;

	fn bossy_command(&self) -> bossy::Command {
		self
			.get_inner_parent()
			.bossy_command()
			.with_arg(Self::SUBCOMMAND_NAME)
	}

	fn version_command(&self) -> bossy::Command {
		self.bossy_command().with_arg("--version")
	}

	fn validate_version(&self) -> bool {
		self
			.version_command()
			.run_and_wait()
			.is_ok_and(|status| status.success())
	}

	fn new(parent: &'src Self::Parent) -> Result<Self> {
		let instance = unsafe { Self::new_unchecked(parent) };
		if !instance.validate_version() {
			Err(Error::VersionCheckFailed)
		} else {
			Ok(instance)
		}
	}
}

#[macro_export]
macro_rules! impl_exec_instance {
	($t:ty, $name:expr) => {
		impl $crate::shared::ExecInstance for $t {
			const BINARY_NAME: &'static str = $name;

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
			pub fn new() -> $crate::error::Result<Self> {
				$crate::shared::ExecInstance::new()
			}
		}
	};
}

#[macro_export]
macro_rules! impl_exec_child {
	($t:ty, parent = $parent:ty, subcommand = $name:expr) => {
		impl<'src> $crate::shared::ExecChild<'src> for $t {
			const SUBCOMMAND_NAME: &'static str = $name;
			type Parent = $parent;

			unsafe fn new_unchecked(parent: &'src Self::Parent) -> Self {
				Self {
					exec_parent: parent,
				}
			}

			fn get_inner_parent(&self) -> &Self::Parent {
				&self.exec_parent
			}
		}

		impl<'src> $t {
			/// Constructs an instance of `Self` using `which`.
			pub fn new(
				parent: &'src <Self as $crate::shared::ExecChild<'src>>::Parent,
			) -> $crate::error::Result<Self> {
				$crate::shared::ExecChild::new(parent)
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
