use openssl::x509::X509;

use super::SecurityCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum FindCertificatesError {
	#[error("Error running `security find-certificate -a -p`: {0}")]
	ExecuteError(#[from] bossy::Error),

	#[error("Error parsing X509 cert: {0}")]
	X509ParseFailed(#[from] openssl::error::ErrorStack),
}

impl SecurityCLIInstance {
	pub fn find_certificates(&self) -> Result<(), FindCertificatesError> {
		let mut command = self
			.bossy_command()
			.with_arg("find-certificate")
			.with_arg("-a")
			.with_arg("-p");
		let output = command.run_and_wait_for_output()?;

		let mut certs = X509::stack_from_pem(output.stdout())?;

		Ok(())
	}
}
