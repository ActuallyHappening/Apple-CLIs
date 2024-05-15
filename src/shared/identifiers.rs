use crate::prelude::*;

pub use device_name::{DeviceName, IPadVariant, IPhoneVariant};
pub use num_generation::NumGeneration;
pub use screen_size::ScreenSize;
pub use model_name::ModelName;

mod device_name;
mod model_name;
mod num_generation;
mod m_gen;
mod generation;
mod screen_size;

/// e.g. "com.apple.CoreSimulator.SimRuntime.iOS-16-4"
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct RuntimeIdentifier(String);
