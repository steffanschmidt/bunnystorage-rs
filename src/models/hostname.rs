use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct Hostname {
	// The unique ID of the hostname
	pub id: i64,
	// The hostname value for the domain name
	pub value: String,
	// Determines if the Force SSL feature is enabled
	#[serde(alias = "ForceSSL")]
	pub force_ssl: bool,
	// Determines if this is a system hostname controlled by bunny.net
	pub is_system_hostname: bool,
	// Determines if the hostname has an SSL certificate configured
	pub has_certificate: bool,
	// Contains the Base64Url encoded certificate for the hostname
	pub certificate: Option<String>,
	// Contains the Base64Url encoded certificate key for the hostname
	pub certificate_key: Option<String>,
}