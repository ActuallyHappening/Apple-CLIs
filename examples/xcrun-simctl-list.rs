//! See `cargo run --example simctl` for a more complete example.

fn main() -> Result<(), apple_clis::error::Error> {
	let xcrun_instance = apple_clis::xcrun::XcRunInstance::new()?;
	let simctl_instance = xcrun_instance.simctl();

	let output = simctl_instance.list()?;
	println!("Output: {:#?}", output);

	let devices = output.devices();
	for device in devices {
		println!("Device: {:#?}", device);
	}

	Ok(())
}