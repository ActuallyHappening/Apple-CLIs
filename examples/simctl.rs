use apple_clis::{
	open::{well_known::WellKnown, OpenCLIInstance},
	shared::identifiers::{DeviceName, IPadVariant},
	xcrun::{simctl::{boot::{self, BootOutput}, list::ListDevice}, XcRunInstance},
};

fn main() -> Result<(), apple_clis::error::Error> {
	let xcrun_instance = XcRunInstance::new()?;
	let simctl_instance = xcrun_instance.simctl();

	let output = simctl_instance.list()?;
	let devices: Vec<&ListDevice> = output.devices().collect();
	for device in devices {
		println!(
			"Simulator device {name} is {state:?} and is ready = {ready:?}",
			name = device.name,
			state = device.state,
			ready = device.ready()
		);
	}

	let ipad_simulator: &IPadVariant = output
		.ipads()
		.max()
		.expect("an iPad simulator to be available");
	println!("Found an {} simulator", ipad_simulator);

	if let IPadVariant::Pro { size, generation } = ipad_simulator {
		let inches: f32 = size.inches();
		let gen: u8 = generation.get();
		println!(
			"Ooh, its a pro size {} {:?} generation {} {:?}",
			size, inches, generation, gen,
		);
	}

	// boot the simulator
	let boot_result = simctl_instance.boot(DeviceName::from(*ipad_simulator))?;
	match boot_result {
		BootOutput::Success => println!("Booted the simulator"),
		BootOutput::AlreadyBooted => println!("Simulator was already booted"),
		_ => println!("PRs welcome to cover more cases"),
	}

	// open the simulator
	let open_instance = OpenCLIInstance::new()?;
	open_instance.open_well_known(&WellKnown::Simulator)?;

	Ok(())
}
