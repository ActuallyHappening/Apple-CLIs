use camino::Utf8PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Command exited with an error: {0}")]
	ExecuteErrored(#[from] bossy::Error),

	/// TODO: propagate more information
	#[error("Calling `--version` failed")]
	VersionCheckFailed,

	#[error("Error parsing command JSON output: {0}")]
	ParseJson(#[from] serde_json::Error),

	#[error("Failed to parse {}: {:?}", name, err)]
	ParsingFailed {
		/// What was being parsed
		name: String,
		#[source]
		err: nom::Err<nom::error::Error<String>>,
	},

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
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;
