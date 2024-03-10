use apple_clis::Args;
use clap::Parser;
use tracing::*;

fn main() {
		tracing_subscriber::fmt::init();
    let config = Args::parse();

		trace!("Config: {:?}", config);
}
