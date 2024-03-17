use crate::prelude::*;

pub mod identifiers;
pub mod types;

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
			.run_and_wait_for_output()
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
			.run_and_wait_for_output()
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

			fn validate_version(&self) -> bool {
				true
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

			fn validate_version(&self) -> bool {
				true
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

/// [impl]s [std::str::FromStr] for a type that already implements
/// [NomFromStr]
#[macro_export]
macro_rules! nom_from_str {
	($type:ty) => {
		impl std::str::FromStr for $type {
			type Err = $crate::error::Error;

			#[tracing::instrument(level = "trace", skip(input))]
			fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
				match <$type>::nom_from_str(input) {
					Ok((remaining, output)) => {
						if remaining.is_empty() {
							Ok(output)
						} else {
							Err(Error::ParsingRemainingNotEmpty {
								input: input.to_owned(),
								remaining: remaining.to_owned(),
								parsed_debug: format!("{:#?}", output),
							})
						}
					}
					Err(e) => Err(Error::ParsingFailed {
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

#[macro_export]
macro_rules! impl_str_serde {
	($type:ty) => {
		impl<'de> serde::Deserialize<'de> for $type {
			fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
			where
				D: serde::Deserializer<'de>,
			{
				let string = String::deserialize(deserializer)?;
				<$type as std::str::FromStr>::from_str(&string).map_err(serde::de::Error::custom)
			}
		}

		// impl serialize
		impl serde::Serialize for $type {
			fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
			where
				S: serde::Serializer,
			{
				serializer.serialize_str(&self.to_string())
			}
		}
	};
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub(crate) fn ws<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
	inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
	F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
	delimited(multispace0, inner, multispace0)
}

pub(crate) trait SuccessfullyParsed {
	fn successfully_parsed(&self) -> bool;
}

#[cfg(test)]
fn assert_nom_parses<T: NomFromStr + std::fmt::Display + std::fmt::Debug>(
	examples: impl IntoIterator<Item = &'static str>,
	successfully_parsed: impl Fn(&T) -> bool,
) {
	use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

	let fmt_verbose_layer = tracing_subscriber::fmt::layer().pretty();

	tracing_subscriber::registry()
		.with(fmt_verbose_layer)
		.try_init()
		.ok();

	for example in examples.into_iter() {
		let example = example.to_string();
		match T::nom_from_str(&example) {
			Ok((remaining, parsed)) => {
				assert!(
					remaining.is_empty(),
					"Leftover input {:?} while parsing {} into {:?}",
					remaining,
					example,
					parsed
				);
				assert_eq!(
					parsed.to_string(),
					example,
					"Parsing {} into {:?} didn't match Display of {}",
					example,
					parsed,
					example
				);
				assert!(
					successfully_parsed(&parsed),
					"While parsing {}, got unimplemented variant {:?}",
					example,
					parsed
				);
			}
			Err(err) => {
				panic!("Failed to parse {:?}: {}", example, err);
			}
		}
	}
}
