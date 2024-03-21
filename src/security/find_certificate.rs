use crate::prelude::*;

use openssl::x509::X509;
use openssl::{error::ErrorStack as OpenSslError, nid::Nid, x509::X509NameRef};
use serde::Serialize;
use thiserror::Error;

use super::SecurityCLIInstance;

impl SecurityCLIInstance {
	const DEVELOPER_NAME_SCHEMAS: [&'static str; 2] = ["Developer:", "Development:"];

	fn get_pem_list(&self, name_substr: &str) -> bossy::Result<bossy::Output> {
		self
			.bossy_command()
			.with_args(["find-certificate", "-p", "-a", "-c", name_substr])
			.run_and_wait_for_output()
	}

	fn get_developer_pem_list(&self) -> bossy::Result<Vec<bossy::Output>> {
		Self::DEVELOPER_NAME_SCHEMAS
			.iter()
			.map(|name| self.get_pem_list(name))
			.collect()
	}

	pub fn get_developer_pems(&self) -> Result<Vec<X509>> {
		let certs = self
			.get_developer_pem_list()?
			.into_iter()
			.map(|output| X509::stack_from_pem(output.stdout()).map_err(Error::X509ParseFailed))
			.collect::<Result<Vec<_>>>()?;
		let certs = certs.into_iter().flatten();
		Ok(certs.collect())
	}

	#[instrument(ret)]
	pub fn get_developer_certs(&self) -> Result<Vec<Certificate>> {
		Ok(
			self
				.get_developer_pems()?
				.into_iter()
				.filter_map(|cert| Certificate::try_from_x509(cert).ok())
				.collect(),
		)
	}
}

#[derive(Debug, Serialize)]
pub struct Certificate {
	/// e.g. "Apple Development: johnsmith@hotmail.com (UIOH89JLHGF)"
	pub common_name: String,
	// e.g. "John Smith"
	pub organization_name: Option<String>,
}

#[derive(Debug, Error)]
pub enum X509FieldError {
	#[error("Missing X509 field {name:?} ({id:?})")]
	FieldMissing { name: &'static str, id: Nid },

	#[error("Field contained invalid UTF-8: {0}")]
	FieldNotValidUtf8(#[source] OpenSslError),
}

#[tracing::instrument(level = "trace", skip(subject_name, field_name, field_nid))]
fn get_x509_field(
	subject_name: &X509NameRef,
	field_name: &'static str,
	field_nid: Nid,
) -> std::result::Result<String, X509FieldError> {
	subject_name
		.entries_by_nid(field_nid)
		.next()
		.ok_or(X509FieldError::FieldMissing {
			name: field_name,
			id: field_nid,
		})?
		.data()
		.as_utf8()
		.map_err(X509FieldError::FieldNotValidUtf8)
		.map(|s| s.to_string())
}

#[derive(Debug, thiserror::Error)]
pub enum FromX509Error {
	#[error("Common Name missing in cert: {0}")]
	CommonNameMissing(#[from] X509FieldError),
}

impl Certificate {
	#[tracing::instrument(level = "trace", skip(cert))]
	pub fn try_from_x509(cert: X509) -> std::result::Result<Self, FromX509Error> {
		let subject = cert.subject_name();
		let common_name = get_x509_field(subject, "Common Name", Nid::COMMONNAME)?;
		let organization_name = get_x509_field(subject, "Organization", Nid::ORGANIZATIONNAME).ok();
		Ok(Certificate {
			common_name,
			organization_name,
		})
	}
}
