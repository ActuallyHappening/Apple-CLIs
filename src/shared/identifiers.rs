use crate::shared::prelude::*;

use serde::Deserialize;

pub mod device_name;
pub mod generation;
pub mod screen_size;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);
