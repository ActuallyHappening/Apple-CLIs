pub use app_directory::AppDirectory;
mod app_directory {
	use std::str::FromStr;

	use crate::prelude::*;

	/// Represents a path that points to a .app directory
	/// Exists for type-safety and to provide a consistent API
	#[derive(Debug, Serialize, Clone)]
	pub struct AppDirectory(Utf8PathBuf);

	impl AsRef<Utf8Path> for AppDirectory {
		fn as_ref(&self) -> &Utf8Path {
			self.0.as_ref()
		}
	}

	impl AsRef<std::path::Path> for AppDirectory {
		fn as_ref(&self) -> &std::path::Path {
			self.0.as_ref()
		}
	}

	impl FromStr for AppDirectory {
		type Err = Error;

		fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
			Self::new(s)
		}
	}

	impl<'de> serde::Deserialize<'de> for AppDirectory {
		fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
		where
			D: serde::Deserializer<'de>,
		{
			let string = String::deserialize(deserializer)?;
			AppDirectory::from_str(&string).map_err(serde::de::Error::custom)
		}
	}

	impl AppDirectory {
		/// # Safety
		/// Assets that the path exists, and points to a .app bundle directory
		unsafe fn new_unchecked(path: impl AsRef<Utf8Path>) -> Self {
			Self(Utf8PathBuf::from(path.as_ref()))
		}

		pub fn new(path: impl AsRef<Utf8Path>) -> Result<Self> {
			let path = path.as_ref();
			match path.try_exists() {
				Ok(true) => Ok(unsafe { Self::new_unchecked(path) }),
				Ok(false) => Err(Error::PathDoesNotExist {
					path: path.to_owned(),
					err: None,
				}),
				Err(err) => Err(Error::PathDoesNotExist {
					path: path.to_owned(),
					err: Some(err),
				}),
			}
		}

		pub fn get(&self) -> &Utf8Path {
			self.0.as_ref()
		}
	}
}
