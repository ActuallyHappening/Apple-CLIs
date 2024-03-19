use crate::prelude::*;

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

	fn validate_version(&self) -> std::result::Result<bool, bossy::Error> {
		self
			.version_command()
			.run_and_wait_for_output()
			.map(|status| status.success())
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
		match instance.validate_version() {
			Ok(true) => Ok(instance),
			Ok(false) => Err(Error::VersionCheckFailed(None)),
			Err(e) => Err(Error::VersionCheckFailed(Some(e))),
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

	fn validate_version(&self) -> std::result::Result<bool, bossy::Error> {
		self
			.version_command()
			.run_and_wait_for_output()
			.map(|status| status.success())
	}

	fn new(parent: &'src Self::Parent) -> Result<Self> {
		let instance = unsafe { Self::new_unchecked(parent) };
		match instance.validate_version() {
			Ok(true) => Ok(instance),
			Ok(false) => Err(Error::VersionCheckFailed(None)),
			Err(e) => Err(Error::VersionCheckFailed(Some(e))),
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
	($t:ty, $name:expr, skip_version_check) => {
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

			fn validate_version(&self) -> std::result::Result<bool, bossy::Error> {
				Ok(true)
			}
		}

		impl $t {
			/// Constructs an instance of `Self` using `which`.
			pub fn new() -> $crate::error::Result<Self> {
				$crate::shared::ExecInstance::new()
			}
		}
	};
	($t:ty, $name:expr, skip_version_check, extra_flags = $extra_flags:expr) => {
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

			fn bossy_command(&self) -> ::bossy::Command {
				bossy::Command::pure(&self.get_inner_exec_path()).with_args($extra_flags)
			}

			fn validate_version(&self) -> std::result::Result<bool, bossy::Error> {
				Ok(true)
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

pub(crate) trait NomFromStr: Sized {
	fn nom_from_str(input: &str) -> IResult<&str, Self>;
}

impl NomFromStr for NonZeroU8 {
	#[tracing::instrument(level = "trace", skip(input))]
	fn nom_from_str(input: &str) -> IResult<&str, Self> {
		map_res(digit1, |s: &str| s.parse())(input)
	}
}

pub(crate) trait DebugNamed {
	fn name() -> &'static str;
}

/// Used to wrap command output types
pub(crate) trait CommandNomParsable: Sized + std::fmt::Debug + DebugNamed {
	fn success_unimplemented(str: String) -> Self;
	fn error_unimplemented(str: String) -> Self;

	fn success_nom_from_str(input: &str) -> IResult<&str, Self> {
		map(rest, |s: &str| Self::success_unimplemented(s.to_owned()))(input)
	}

	fn success_from_str(input: &str) -> Result<Self> {
		match Self::success_nom_from_str(input) {
			Ok((remaining, output)) => {
				if !remaining.is_empty() {
					tracing::warn!(?remaining, ?output, "Remaining input after parsing a command success output. This likely indicates a potential improvement to the output parser. PRs welcome!")
				}
				Ok(output)
			}
			Err(err) => Err(Error::NomParsingFailed {
				name: Self::name().to_owned(),
				err: err.to_owned(),
			}),
		}
	}

	fn errored_nom_from_str(input: &str) -> IResult<&str, Self> {
		map(rest, |s: &str| Self::error_unimplemented(s.to_owned()))(input)
	}

	fn errored_from_str(input: &str) -> Result<Self> {
		match Self::errored_nom_from_str(input) {
			Ok((remaining, output)) => {
				if !remaining.is_empty() {
					tracing::warn!(?remaining, ?output, "Remaining input after parsing a command error output. This likely indicates a potential improvement to the output parser. PRs welcome!")
				}
				Ok(output)
			}
			Err(err) => Err(Error::NomParsingFailed {
				name: Self::name().to_owned(),
				err: err.to_owned(),
			}),
		}
	}

	fn from_bossy_result(result: bossy::Result<Output>) -> Result<Self> {
		let (success, string) = match result {
			Ok(output) => (true, output.stdout_str()?.to_owned()),
			Err(err) => match err.output() {
				Some(output) => (false, output.stderr_str()?.to_owned()),
				None => Err(Error::CannotLocateStderrStream { err })?,
			},
		};
		if success {
			Self::success_from_str(&string)
		} else {
			Self::errored_from_str(&string)
		}
	}
}

/// [impl]s [std::str::FromStr] for a type that already implements
/// [NomFromStr]
#[macro_export]
macro_rules! impl_from_str_nom {
		($type:ty) => {
			impl std::str::FromStr for $type {
				type Err = $crate::error::Error;

				#[tracing::instrument(level = "trace", skip(input))]
				fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
					match <$type>::nom_from_str(input) {
						Ok((remaining, output)) => {
							if !remaining.is_empty() {
								tracing::warn!(?remaining, ?output, "Remaining input after parsing. This likely indicates a potential improvement to the output parser. PRs welcome!")
							}
							// if remaining.is_empty() {
								Ok(output)
							// } else {
								// Err(Error::ParsingRemainingNotEmpty {
									// input: input.to_owned(),
							// 		remaining: remaining.to_owned(),
							// 		parsed_debug: format!("{:#?}", output),
							// 	})
							// }
						}
						Err(e) => Err(Error::NomParsingFailed {
							err: e.to_owned(),
							name: stringify!($type).into(),
						}),
					}
				}
			}
		};

		($type:ty, unimplemented = $unimplemented:expr) => {
			$crate::nom_from_str!($type);

			impl $crate::shared::SuccessfullyParsed for $type {
				fn successfully_parsed(&self) -> bool {
					matches!(self, $unimplemented)
				}
			}
		};
	}