use crate::prelude::*;

pub use device_name::{DeviceName, IPadVariant, IPhoneVariant};
pub use generation::*;
pub use screen_size::{ScreenSize, MaybeScreenSize};
pub use model_name::ModelName;

mod device_name;
mod model_name;
mod generation;
mod screen_size;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);
