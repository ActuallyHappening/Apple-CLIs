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

	#[error("The hard coded path {:?} was not found ({:?}: {err:?})", hard_coded_path, err)]
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

	#[error("Error find with `which`: {0}")]
	CannotFindWithWhich(#[from] which::Error)
}

pub type Result<T> = std::result::Result<T, crate::error::Error>;
