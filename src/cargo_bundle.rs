use bossy::ExitStatus;

use crate::prelude::*;

pub struct CargoBundleInstance {
	cargo_path: Utf8PathBuf,
	// manifest_dir: Utf8PathBuf,
}

impl CargoBundleInstance {
	/// Path to cargo executable
	pub fn new(
		cargo_path: impl AsRef<Utf8Path>,
		// manifest_dir: impl AsRef<Utf8Path>,
	) -> CargoBundleInstance {
		CargoBundleInstance {
			cargo_path: cargo_path.as_ref().to_path_buf(),
			// manifest_dir: manifest_dir.as_ref().to_path_buf(),
		}
	}

	pub fn try_new_from_which(// manifest_dir: impl AsRef<Utf8Path>,
	) -> Result<CargoBundleInstance> {
		let path = which::which("cargo-bundle")?;
		Ok(CargoBundleInstance::new(
			Utf8PathBuf::try_from(path)?,
			// manifest_dir,
		))
	}

	fn bossy_command(&self) -> bossy::Command {
		bossy::Command::pure(&self.cargo_path).with_arg("bundle")
	}
}

#[derive(thiserror::Error, Debug)]
pub enum BundleError {
	#[error("Error running `cargo bundle`: {0}")]
	ExecuteError(#[from] bossy::Error),
}

impl CargoBundleInstance {
	// cargo bundle --target aarch64-apple-ios-sim
	pub fn bundle_ios(&self) -> Result<ExitStatus> {
		let exit_status = self
			.bossy_command()
			.with_args(["--target", "aarch64-apple-ios-sim"])
			.run_and_wait()?;

		Ok(exit_status)
	}
}
