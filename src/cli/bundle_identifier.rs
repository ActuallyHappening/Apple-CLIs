use clap::Args;

use crate::prelude::*;

#[derive(Args, Debug)]
pub struct BundleIdentifierArgs {
	#[clap(long)]
	bundle_id: String,
}

impl BundleIdentifierArgs {
	pub fn resolve(self) -> color_eyre::Result<String> {
		Ok(self.bundle_id)
	}
}