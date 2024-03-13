use crate::prelude::*;

use openssl::x509::X509;
use openssl::{error::ErrorStack as OpenSslError, nid::Nid, x509::X509NameRef};
use thiserror::Error;
use tracing::{error, info, warn};

use super::SecurityCLIInstance;

#[derive(thiserror::Error, Debug)]
pub enum FindCertificatesError {
	#[error("Error running `security find-certificate -a -p`: {0}")]
	ExecuteError(#[from] bossy::Error),

	#[error("Error parsing X509 cert: {0}")]
	X509ParseFailed(#[from] openssl::error::ErrorStack),
}

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

	pub fn get_developer_pems(&self) -> Result<Vec<X509>, FindCertificatesError> {
		let certs = self
			.get_developer_pem_list()?
			.into_iter()
			.map(|output| {
				X509::stack_from_pem(output.stdout()).map_err(FindCertificatesError::X509ParseFailed)
			})
			.collect::<Result<Vec<_>, _>>()?;
		let certs = certs.into_iter().flatten();
		Ok(certs.collect())
	}

	pub fn get_developer_certs(&self) -> Result<Vec<Certificate>, FindCertificatesError> {
		Ok(
			self
				.get_developer_pems()?
				.into_iter()
				.filter_map(|cert| Certificate::try_from_x509(cert).ok())
				.collect(),
		)
	}
}

#[derive(Debug)]
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

fn get_x509_field(
	subject_name: &X509NameRef,
	field_name: &'static str,
	field_nid: Nid,
) -> Result<String, X509FieldError> {
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
	pub fn try_from_x509(cert: X509) -> Result<Self, FromX509Error> {
		let subject = cert.subject_name();
		let common_name = get_x509_field(subject, "Common Name", Nid::COMMONNAME)?;
		let organization_name = get_x509_field(subject, "Organization", Nid::ORGANIZATIONNAME).ok();
		Ok(Certificate {
			common_name,
			organization_name,
		})
	}
}

mod teams {
	use std::collections::BTreeSet;

	use once_cell_regex::regex;
	use openssl::{nid::Nid, x509::X509};
	use tracing::{error, info, warn};

	use crate::security::SecurityCLIInstance;

	use super::{get_x509_field, FindCertificatesError, X509FieldError};

	impl SecurityCLIInstance {
		#[deprecated(note = "Use `get_developer_certs` instead")]
		pub fn get_developer_teams(&self) -> Result<Vec<Team>, FindCertificatesError> {
			let certs = self.get_developer_pems()?;
			Ok(
				certs
					.into_iter()
					.flat_map(|cert| {
						Team::from_x509(cert).map_err(|err| {
							error!("{}", err);
							err
						})
					})
					// Silly way to sort this and ensure no dupes
					.collect::<BTreeSet<_>>()
					.into_iter()
					.collect(),
			)
		}
	}

	#[derive(Debug, thiserror::Error)]
	pub enum FromX509Error {
		#[error("skipping cert: {0}")]
		CommonNameMissing(#[source] X509FieldError),
		#[error("skipping cert {common_name:?}: {source}")]
		OrganizationalUnitMissing {
			common_name: String,
			source: X509FieldError,
		},
	}

	#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
	pub struct Team {
		pub name: String,
		pub id: String,
	}

	impl Team {
		pub fn from_x509(cert: X509) -> Result<Self, FromX509Error> {
			let subj = cert.subject_name();
			let common_name = get_x509_field(subj, "Common Name", Nid::COMMONNAME)
				.map_err(FromX509Error::CommonNameMissing)?;
			let organization = get_x509_field(subj, "Organization", Nid::ORGANIZATIONNAME);
			let name = if let Ok(organization) = organization {
				info!(
					"found cert {:?} with organization {:?}",
					common_name, organization
				);
				organization
			} else {
				warn!(
					"found cert {:?} but failed to get organization; falling back to displaying common name",
					common_name
				);
				regex!(r"Apple Develop\w+: (.*) \(.+\)")
			          .captures(&common_name)
			          .map(|caps| caps[1].to_owned())
			          .unwrap_or_else(|| {
			              warn!("regex failed to capture nice part of name in cert {:?}; falling back to displaying full name", common_name);
			              common_name.clone()
			          })
			};
			let id = get_x509_field(subj, "Organizational Unit", Nid::ORGANIZATIONALUNITNAME).map_err(
				|source| FromX509Error::OrganizationalUnitMissing {
					common_name,
					source,
				},
			)?;
			Ok(Self { name, id })
		}
	}
}
