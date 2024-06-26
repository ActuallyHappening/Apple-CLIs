use camino::Utf8PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Command exited with an error: {0}")]
	ExecuteErrored(#[from] bossy::Error),

	/// Gained by calling an `success` method on a command output
	#[error("The output of a command did not succeed as expected")]
	OutputErrored {
		debug_msg: String,
		help_hint: Option<String>,
	},

	/// TODO: propagate more information
	#[error("Calling `--version` failed")]
	VersionCheckFailed(#[source] Option<bossy::Error>),

	#[error("Error parsing command JSON output: {0}")]
	ParseJson(#[from] serde_json::Error),

	/// Used for [nom] parsing errors
	#[error("Failed to parse {}: {:?}", name, err)]
	NomParsingFailed {
		/// What was being parsed
		name: String,
		#[source]
		err: nom::Err<nom::error::Error<String>>,
	},

	/// Used for [nom] parsing errors
	#[error(
		"The parsed string was not completely consumed, with {:?} left from {:?}. Parsed: {:?}",
		input,
		remaining,
		parsed_debug
	)]
	ParsingRemainingNotEmpty {
		input: String,
		remaining: String,
		/// [Debug] representation of parsed value
		parsed_debug: String,
	},

	/// [crate::open::well_known::WellKnown] was unable to locate the path on disk
	#[error(
		"The hard coded path {:?} was not found ({:?}: {err:?})",
		hard_coded_path,
		err
	)]
	WellKnownPathNotFound {
		hard_coded_path: Utf8PathBuf,
		err: Option<std::io::Error>,
	},

	#[error("Path does not exist: {path} ({err:?}: {err:?})")]
	PathDoesNotExist {
		path: Utf8PathBuf,
		err: Option<std::io::Error>,
	},

	#[error("Error converting path to UTF-8: {0}")]
	PathNotUtf8(#[from] camino::FromPathBufError),

	#[error("Error parsing X509 cert: {0}")]
	X509ParseFailed(#[from] openssl::error::ErrorStack),

	#[error(
		"Error trying to parse the `codesign` status of a .app: Missing property {}",
		missing_key
	)]
	SigningPropertyNotFound { missing_key: String },

	#[error("Error parsing date: {0}")]
	ParseDateError(#[from] time::error::Parse),

	#[error("Error find with `which`: {0}")]
	CannotFindWithWhich(#[from] which::Error),

	#[error("Error finding .app directory: {err:?} at {path}")]
	AppDirectoryConstructorError {
		path: Utf8PathBuf,
		err: Option<std::io::Error>,
	},

	#[error("Couldn't locate the stderr output stream even though the command errored: {err}")]
	CannotLocateStderrStream { err: bossy::Error },
}

impl Error {
	pub(crate) fn output_errored(debug_msg: impl std::fmt::Debug) -> Self {
		Self::OutputErrored {
			debug_msg: format!("{:#?}", debug_msg),
			help_hint: None,
		}
	}

	pub(crate) fn output_errored_with_hint(
		debug_msg: impl std::fmt::Debug,
		help_hint: impl ToString,
	) -> Self {
		Self::OutputErrored {
			debug_msg: format!("{:#?}", debug_msg),
			help_hint: Some(help_hint.to_string()),
		}
	}
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;
