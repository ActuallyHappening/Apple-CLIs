//! See https://developer.apple.com/documentation/xcode/creating-distribution-signed-code-for-the-mac

use crate::{impl_exec_instance, prelude::*};

pub mod display;
pub mod sign;

pub struct CodesignCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(CodesignCLIInstance, "codesign");
