use apple_clis::ios_deploy::{detect::DetectDevicesConfig, IosDeployCLIInstance};

fn main() -> Result<(), apple_clis::error::Error> {
	tracing_subscriber::fmt::init();
	let ios_deploy = IosDeployCLIInstance::new().expect("Couldn't find ios-deploy executable");
	let detected_devices = ios_deploy
		.detect_devices(&DetectDevicesConfig::default())
		.expect("Couldn't detect devices");

	println!("Devices: {:?}", detected_devices);

	Ok(())
}
