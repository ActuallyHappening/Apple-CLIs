use crate::prelude::*;

/// Arguments to specify a path to a .app directory
#[derive(clap::Args, Debug)]
#[group(required = true, multiple = false)]
pub struct AppPathArgs {
	/// Provide an absolute path to the .app file
	#[arg(long, long = "path")]
	app_path: Option<types::AppDirectory>,

	/// Use glob to find the first .app file in the current directory or any subdirectories
	#[arg(long)]
	glob: bool,
}

impl AppPathArgs {
	#[tracing::instrument(level = "trace", skip(self))]
	pub fn resolve(self) -> color_eyre::Result<types::AppDirectory> {
		let path = match self.app_path {
			Some(p) => p,
			None => match self.glob {
				false => Err(eyre!(
					"Clap should have enforced that either `app_path` or `glob` was set"
				))?,
				true => {
					let matches = glob::glob("**/*.app")
						.map_err(|err| eyre!("Error running glob: {}", err))?
						.filter_map(|p| p.ok())
						.filter_map(|p| Utf8PathBuf::try_from(p).ok())
						.collect::<Vec<_>>();

					if matches.len() > 1 {
						warn!(
							globbed = ?matches,
							"More than one .app file found, using the first match",
						);
					}

					match matches.first() {
						Some(p) => {
							info!(message = "Using the first matched .app file", "match" = ?p);
							types::AppDirectory::new(p)?
						}
						None => Err(eyre!(
							"No .app files found in the current directory or any subdirectories"
						))?,
					}
				}
			},
		};
		if !path.get().exists() {
			Err(eyre!("Provided app path does not exist: {:?}", path))?
		}
		Ok(path)
	}
}
