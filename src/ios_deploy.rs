use crate::{impl_exec_instance, prelude::*};

pub mod detect;
pub mod upload;

#[derive(Debug)]
pub struct IosDeployCLIInstance {
	exec_path: Utf8PathBuf,
}

impl_exec_instance!(IosDeployCLIInstance);