pub mod prelude {
	pub(crate) use crate::impl_exec_instance;
	pub use crate::shared::prelude::*;
	pub use crate::error::{Error, Result};
	pub use crate::error as error;
	pub use camino::{Utf8Path, Utf8PathBuf};
	pub use serde::Deserialize;
	pub use std::collections::HashMap;
	pub use std::num::NonZeroU8;
	pub use std::fmt::Display;
}

pub mod error;
pub mod shared;

pub mod cargo_bundle;
pub mod cli;
pub mod codesign;
pub mod ios_deploy;
pub mod pkgbuild;
pub mod pkgutil;
pub mod productbuild;
pub mod productsign;
pub mod security;
pub mod spctl;
pub mod xcodebuild;
pub mod xcrun;
pub mod open;