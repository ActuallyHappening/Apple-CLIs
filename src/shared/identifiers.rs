use crate::shared::prelude::*;

use serde::Deserialize;

pub use device_name::{DeviceName, IPadVariant, IPhoneVariant};
pub use generation::Generation;
pub use screen_size::ScreenSize;

mod device_name;
mod generation;
mod screen_size;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);
