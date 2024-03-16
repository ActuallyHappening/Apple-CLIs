use apple_clis::security;

#[tracing::instrument(level = "trace", skip())]
fn main() {
	let security_instance =
		security::SecurityCLIInstance::new().expect("Couldn't find security executable");

	let teams = security_instance
		.get_developer_certs()
		.expect("Couldn't get developer teams");
	println!(
		"{} development teams found with `security`: {:?}",
		teams.len(),
		teams
	);

	let pems = security_instance
		.get_developer_pems()
		.expect("Couldn't get developer pems");
	println!(
		"{} development pems found with `security`: {:#?}",
		pems.len(),
		pems
	);
}
