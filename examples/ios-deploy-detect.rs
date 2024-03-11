use apple_clis::ios_deploy::IosDeployCLIInstance;

fn main() {
	tracing_subscriber::fmt::init();
	let ios_deploy = IosDeployCLIInstance::try_new_from_which().expect("Couldn't find ios-deploy executable");
	let detected_devices = ios_deploy.detect_devices().expect("Couldn't detect devices");

	println!("Devices: {:?}", detected_devices);
}
