use apple_clis::ios_deploy::IosDeployInstance;

fn main() {
	tracing_subscriber::fmt::init();
	let ios_deploy = IosDeployInstance::try_new_from_which().expect("Couldn't find ios-deploy executable");
	let detected_devices = ios_deploy.detect_devices().expect("Couldn't detect devices");

	println!("Devices: {:?}", detected_devices);
}
