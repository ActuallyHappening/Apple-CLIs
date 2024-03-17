pub use app_directory::AppDirectory;
mod app_directory {
	use crate::prelude::*;

	/// Represents a path that points to a .app directory
	/// Exists for type-safety and to provide a consistent API
	#[derive(Debug)]
	pub struct AppDirectory(Utf8PathBuf);

	impl AsRef<Utf8Path> for AppDirectory {
		fn as_ref(&self) -> &Utf8Path {
			self.0.as_ref()
		}
	}

	impl AppDirectory {
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
	}
}
