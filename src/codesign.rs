//! See <https://developer.apple.com/documentation/xcode/creating-distribution-signed-code-for-the-mac>

use crate::prelude::*;

pub mod display;
pub mod sign;


#[derive(Debug)]
pub struct CodesignCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(
	CodesignCLIInstance,
	"codesign",
	skip_version_check,
	extra_flags = ["-vvvv"]
);
