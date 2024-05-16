use apple_clis::prelude::*;

fn main() -> Result<(), apple_clis::error::Error> {
	let xcrun_instance = xcrun::XcRunInstance::new()?;
	let simctl_instance = xcrun_instance.simctl();

	let output = simctl_instance.list()?;
	let devices: Vec<&simctl::list::ListDevice> = output.success()?.devices().collect();
	for device in devices {
		println!(
			"Simulator device {name} is {state:?} and is ready = {ready:?}",
			name = device.name,
			state = device.state,
			ready = device.ready()
		);
	}

	// the .names() and .ipads() are implemented on extension traits imported with
	// use apple_clis::prelude::*;
	// to make finding devices easier and more ergonomic
	let ipad_simulator: &IPadVariant = output
		.get_success()
		.expect("to succeed")
		.devices()
		.names()
		.ipads()
		.max()
		.expect("an iPad simulator to be available");
	println!("Found an {} simulator", ipad_simulator);

	if let IPadVariant::Pro {
		size, generation, ..
	} = ipad_simulator
	{
		let inches: f32 = size.inches();
		let gen: u8 = match generation {
			// generations can be M1 or gen 6
			Generation::Num(num) => num.get(),
			Generation::M(m_gen) => m_gen.get_u8(),
		};
		println!(
			"Ooh, its a pro size {} {:?} generation {} {:?}",
			size, inches, generation, gen,
		);
	}

	// boot the simulator
	let boot_result = simctl_instance.boot(DeviceName::from(*ipad_simulator))?;
	match boot_result {
		simctl::boot::BootOutput::SuccessUnImplemented { .. } => println!("Booted the simulator"),
		simctl::boot::BootOutput::AlreadyBooted => println!("Simulator was already booted"),
		_ => println!("PRs welcome to cover more cases"),
	}

	// open the simulator
	let open_instance = open::OpenCLIInstance::new()?;
	open_instance.open_well_known(&open::well_known::WellKnown::Simulator)?;

	Ok(())
}
