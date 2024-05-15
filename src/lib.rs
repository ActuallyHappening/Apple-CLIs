pub mod prelude {
	// public exports
	pub use crate::error::Error;
	pub use crate::shared::identifiers::*;
	pub use crate::{
		codesign, error, ios_deploy, open, security, shared, shared::identifiers, shared::types, spctl,
		xcrun, xcrun::simctl,
	};

	// trait exts
	pub use crate::shared::PublicCommandOutput;
	pub use crate::xcrun::simctl::list::{ListDeviceNamesExt, ListDevicesExt};

	// dep re-exports
	pub use bossy;
	pub use camino;

	// internal re-exports
	pub(crate) use crate::error::Result;
	pub(crate) use crate::shared::ExecInstance;
	pub(crate) use crate::shared::{impl_exec_child, impl_exec_instance, impl_from_str_nom};
	pub(crate) use crate::shared::{ws, CommandNomParsable, NomFromStr};
	pub(crate) use crate::include_doc;

	// internal dep imports
	pub(crate) use bossy::{ExitStatus, Output};
	pub(crate) use camino::{Utf8Path, Utf8PathBuf};
	#[allow(unused_imports)]
	pub(crate) use nom::{
		branch::{alt, permutation},
		bytes::complete::{tag, take_till, take_till1, take_until, take_while},
		character::complete::{alpha0, alpha1, digit1, multispace0, multispace1, space0, space1},
		combinator::{all_consuming, cut, map, map_res, peek, rest, success, value},
		multi::fold_many1,
		number::complete::float,
		sequence::{delimited, pair, preceded, terminated, tuple},
		IResult,
	};
	pub(crate) use serde::{Deserialize, Serialize};
	pub(crate) use std::borrow::Cow;
	pub(crate) use std::collections::HashMap;
	pub(crate) use std::fmt::Display;
	pub(crate) use std::num::NonZeroU8;
	pub(crate) use std::str::FromStr;
	pub(crate) use strum::EnumDiscriminants;
	pub(crate) use tracing::{debug, error, info, instrument, trace, warn};

	// cli only exports
	#[cfg(feature = "cli")]
	pub(crate) use clap::{Args, Parser, Subcommand, ValueEnum};
	#[cfg(feature = "cli")]
	pub(crate) use color_eyre::{eyre::eyre, Section};
}

#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod shared;

// pub mod cargo_bundle;
pub mod codesign;
pub mod ios_deploy;
pub mod open;
pub mod security;
pub mod spctl;
pub mod xcrun;

// pub mod xcodebuild;
// pub mod pkgbuild;
// pub mod hdiutil;
// pub mod pkgutil;
// pub mod productbuild;
// pub mod productsign;

macro_rules! include_doc {
		(cmd_error) => {
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_error.md"))
		};
		(cmd_success) => {
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/command_success.md"))
		};
		(must_use_cmd_output) => {
			include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/inline/must_use_command_output.md"))
		};
}
pub(crate) use include_doc;