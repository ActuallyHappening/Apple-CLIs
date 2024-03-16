pub mod prelude {
	pub use crate::error;
	pub use crate::error::{Error, Result};
	pub use crate::shared::prelude::*;
	pub(crate) use crate::{impl_exec_child, impl_exec_instance};
	pub use camino::{Utf8Path, Utf8PathBuf};
	pub use serde::{Deserialize, Serialize};
	pub use std::collections::HashMap;
	pub use std::fmt::Display;
	pub use std::num::NonZeroU8;
	pub use tracing::{debug, error, info, instrument, trace, warn};
}

#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod shared;

pub mod cargo_bundle;
pub mod codesign;
pub mod ios_deploy;
pub mod open;
pub mod pkgbuild;
pub mod pkgutil;
pub mod productbuild;
pub mod productsign;
pub mod security;
pub mod spctl;
pub mod xcodebuild;
pub mod xcrun;
