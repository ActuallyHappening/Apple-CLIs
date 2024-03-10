fn main() {
	let real_devices = apple_clis::list_real::list_real();

	println!("Devices: {:?}", real_devices);
}