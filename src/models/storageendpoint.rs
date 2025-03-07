use std::fmt::Display;

use crate::errors::Error;

pub enum StorageEndpoint {
	Falkenstein,
	London,
	NewYork,
	LosAngeles,
	SingaPore,
	Stockholm,
	SaoPaulo,
	Johannesburg,
	Sydney,
}

impl Display for StorageEndpoint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let storage_endpoint_name: &str = match self {
			StorageEndpoint::Falkenstein => "storage.bunnycdn.com",
			StorageEndpoint::London => "uk.storage.bunnycdn.com",
			StorageEndpoint::NewYork => "ny.storage.bunnycdn.com",
			StorageEndpoint::LosAngeles => "la.storage.bunnycdn.com",
			StorageEndpoint::SingaPore => "sg.storage.bunnycdn.com",
			StorageEndpoint::Stockholm => "se.storage.bunnycdn.com",
			StorageEndpoint::SaoPaulo => "br.storage.bunnycdn.com",
			StorageEndpoint::Johannesburg => "jh.storage.bunnycdn.com",
			StorageEndpoint::Sydney => "syd.storage.bunnycdn.com",
		};
		return write!(f, "{}", storage_endpoint_name);
	}
}

impl StorageEndpoint {

	pub fn url(&self) -> String {
		return format!("https://{}", self.to_string());
	}

	pub fn from_str(storage_endpoint_name: &str) -> Result<StorageEndpoint, Error> {
		let storage_endpoint = match storage_endpoint_name {
			"storage.bunnycdn.com" => StorageEndpoint::Falkenstein,
			"uk.storage.bunnycdn.com" => StorageEndpoint::London,
			"ny.storage.bunnycdn.com" => StorageEndpoint::NewYork,
			"la.storage.bunnycdn.com" => StorageEndpoint::LosAngeles,
			"sg.storage.bunnycdn.com" => StorageEndpoint::SingaPore,
			"se.storage.bunnycdn.com" => StorageEndpoint::Stockholm,
			"br.storage.bunnycdn.com" => StorageEndpoint::SaoPaulo,
			"jh.storage.bunnycdn.com" => StorageEndpoint::Johannesburg,
			"syd.storage.bunnycdn.com" => StorageEndpoint::Sydney,
			_ => return Err(Error::new_from_message(
				format!("Invalid Endpoint Name - Provided {}", storage_endpoint_name).as_str()
			)),
		};
		return Ok(storage_endpoint);
	}
}

#[cfg(test)]
mod storage_endpoints_tests {
	use super::*;

	// #[test]
	// fn test_get_url() -> 

	#[test]
	fn test_from_str() {
		let mut test_storage_endpoint_names = Vec::<&str>::new();
		let invalid_storage_endpoint_name = "InvalidStorageEndpoint";
		test_storage_endpoint_names.push("storage.bunnycdn.com");
		test_storage_endpoint_names.push("uk.storage.bunnycdn.com");
		test_storage_endpoint_names.push("ny.storage.bunnycdn.com");
		test_storage_endpoint_names.push("la.storage.bunnycdn.com");
		test_storage_endpoint_names.push("sg.storage.bunnycdn.com");
		test_storage_endpoint_names.push("se.storage.bunnycdn.com");
		test_storage_endpoint_names.push("br.storage.bunnycdn.com");
		test_storage_endpoint_names.push("jh.storage.bunnycdn.com");
		test_storage_endpoint_names.push("syd.storage.bunnycdn.com");
		test_storage_endpoint_names.push("syd.storage.bunnycdn.com");
		test_storage_endpoint_names.push(invalid_storage_endpoint_name);
		for test_storage_endpoint_name in test_storage_endpoint_names.iter() {
			let test_storage_endpoint_name_result = StorageEndpoint::from_str(test_storage_endpoint_name);
			if *test_storage_endpoint_name != invalid_storage_endpoint_name {
				assert!(test_storage_endpoint_name_result.is_ok());
			} else {
				assert!(test_storage_endpoint_name_result.is_err());
			}
		}
	}
}
