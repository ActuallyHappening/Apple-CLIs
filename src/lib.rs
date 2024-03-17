pub mod prelude {
	pub use crate::error;
	pub use crate::error::{Error, Result};
	pub(crate) use crate::{impl_exec_child, impl_exec_instance, impl_str_serde, nom_from_str};
	pub use camino::{Utf8Path, Utf8PathBuf};
	pub use serde::{Deserialize, Serialize};
	pub(crate) use std::collections::HashMap;
	pub(crate) use std::fmt::Display;
	pub(crate) use std::num::NonZeroU8;
	pub(crate) use tracing::{debug, error, info, instrument, trace, warn};
	pub use bossy::{Command, ExitStatus, Output};
	pub use color_eyre::{eyre::eyre, Section};

	pub use crate::shared::identifiers::*;
	pub(crate) use crate::shared::ExecInstance;
	pub(crate) use crate::shared::{ws, NomFromStr};
	#[allow(unused_imports)]
	pub(crate) use nom::{
		branch::{alt, permutation},
		bytes::complete::{tag, take_till, take_until},
		character::complete::{alpha0, alpha1, digit1, space0, space1, multispace0, multispace1},
		combinator::{all_consuming, map, map_res, peek, rest, success, value, cut},
		number::complete::float,
		sequence::{tuple, pair},
		sequence::{delimited, preceded, terminated},
		IResult,
	};
	pub(crate) use strum::EnumDiscriminants;
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
