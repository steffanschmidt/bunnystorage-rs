use std::collections::HashMap;

use serde::Deserialize;
use serde_json::Value;

use crate::{errors::Error, models::storageendpoint::StorageEndpoint};

pub mod statistics;
pub mod regions;
pub mod files;
pub mod apikey;

const BUNNY_STORAGE_API_ROOT: &str = "https://api.bunny.net";
const ENV_BUNNY_STORAGE_API_KEY_NAME: &str = "BUNNYSTORAGE_API_KEY";
const ENV_BUNNY_STORAGE_READ_PASSWORD_NAME: &str = "BUNNYSTORAGE_READ_PASSWORD";
const ENV_BUNNY_STORAGE_WRITE_PASSWORD_NAME: &str = "BUNNYSTORAGE_WRITE_PASSWORD";
const ENV_BUNNY_STORAGE_ZONE_NAME_NAME: &str = "BUNNYSTORAGE_STORAGE_ZONE_NAME";
const ENV_BUNNY_STORAGE_ENDPOINT_NAME_NAME: &str = "BUNNYSTORAGE_ENDPOINT_NAME";

/*

	Note: 
		The write password is optional, since it may not be necessary for to do any
		writing. However in case one wishes to 
*/
pub struct BunnyCDNClientConfig {
	pub api_key: String,
	pub read_password: String,
	pub write_password: Option<String>,
	pub endpoint: StorageEndpoint,
	pub storage_zone_name: String,
}

fn get_string_from_env(env_key: &str) -> Result<String, Error> {
	return std::env::var(env_key)
		.map_err(|env_key_error| Error::new_from_message(&format!(
			"Failed retrieving environment variable. Check {} in .env - Error {}",
			env_key,
			env_key_error.to_string())
		));
}

fn get_non_empty_string_from_env(env_key: &str) -> Result<String, Error> {
	let env_content = get_string_from_env(env_key)?;
	let trimmed_env_content = env_content.trim();
	if trimmed_env_content.is_empty() {
		return Err(Error::new_from_message(&format!(
			"Invalid environment content. Must not be empty. Check {}",
			env_key
		)));
	}
	return Ok(trimmed_env_content.to_string());
}

impl BunnyCDNClientConfig {

	pub fn new_from_env() -> Result<BunnyCDNClientConfig, Error> {
		// Initialize the environment so in case a .env is present
		// then the information is available
		let initialize_env_result = dotenvy::dotenv();
		if let Err(initialize_env_error) = initialize_env_result {
			return Err(Error::new_from_message(&format!(
				"Failed Initializing Environment - Error {}",
				initialize_env_error.to_string())
			));
		}
		// Get the API Key from the environment
		let api_key = get_non_empty_string_from_env(ENV_BUNNY_STORAGE_API_KEY_NAME)?;
		// Get the Read Password from the environment
		let read_password = get_non_empty_string_from_env(ENV_BUNNY_STORAGE_READ_PASSWORD_NAME)?;
		// Get the Storage Endpoint Name from the Environment - This must match with the read/write password
		let endpoint_name = get_non_empty_string_from_env(ENV_BUNNY_STORAGE_ENDPOINT_NAME_NAME)?;
		let endpoint = StorageEndpoint::from_str(&endpoint_name)?;
		// Get the Storage Zone Name
		let storage_zone_name = get_non_empty_string_from_env(ENV_BUNNY_STORAGE_ZONE_NAME_NAME)?;
		// Get the Write Password - This is optional, since it may not be needed depending
		// on the use case
		let mut write_password = String::new();
		let write_password_result = std::env::var(ENV_BUNNY_STORAGE_WRITE_PASSWORD_NAME);
		if let Ok(provided_write_password) = write_password_result {
			write_password = provided_write_password;
		}
		let client_config = BunnyCDNClientConfig{
			api_key,
			read_password,
			write_password: Some(write_password),
			endpoint,
			storage_zone_name,
		};
		return Ok(client_config);
	}

	pub fn valid(&self) -> Result<(), Error> {
		if self.api_key.is_empty() {
			return Err(Error::new_from_message("Invalid API Key"));
		}
		if self.read_password.is_empty() {
			return Err(Error::new_from_message("Invalid Read Password Key"));
		}
		if self.endpoint.url().is_empty() {
			return Err(Error::new_from_message("Invalid Endpoint"));
		}
		if self.storage_zone_name.is_empty() {
			return Err(Error::new_from_message("Invalid Storage Zone Name"));
		}
		return Ok(());
	}
}

#[allow(dead_code)]
pub struct BunnyCDNClient {
	config: BunnyCDNClientConfig,
	http_client: reqwest::Client,
}

pub struct BunnyCDNPageParameters {
	pub page: Option<i32>,
	pub per_page: Option<i32>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct BunnyCDNPageMeta {
	pub items: Vec<Value>,
	pub current_page: u32,
	pub total_items: u32,
	pub has_more_items: bool,
}

impl BunnyCDNPageMeta {
	
	pub fn new() -> BunnyCDNPageMeta {
		return BunnyCDNPageMeta{
			items: Vec::new(),
			current_page: 0,
			total_items: 0,
			has_more_items: false,
		}
	}

	pub fn valid(&self) -> bool {
		return self.current_page > 0;
	}
}

pub struct BunnyCDNGetResponse {
	pub raw_data: String,
	pub page_meta: BunnyCDNPageMeta
}

impl BunnyCDNClient {
	pub fn new(config: BunnyCDNClientConfig) -> Result<BunnyCDNClient, Error> {
		config.valid()?;
		let client = BunnyCDNClient{
			config,
			http_client: reqwest::Client::new(),
		};
		return Ok(client);
	}

	pub fn new_from_env() -> Result<BunnyCDNClient, Error> {
		let client_config = BunnyCDNClientConfig::new_from_env()?;
		return Self::new(client_config);
	}

	fn add_page_parameters(&self, params: &mut HashMap<&str, String>, page_params_opt: Option<&BunnyCDNPageParameters>) {
		if let Some(page_params) = page_params_opt {
			if let Some(page) = page_params.page {
				params.insert("page", page.to_string());
			}
			if let Some(per_page) = page_params.per_page {
				params.insert("perPage", per_page.to_string());
			}
		}
	}

	async fn get(&self, url: &str, access_key: &str, params: Option<&HashMap<&str, String>>) -> Result<BunnyCDNGetResponse, Error> {
		let http_response = self.http_client.get(url)
			.header("AccessKey", access_key)
			.query(&params)
			.send()
			.await
			.map_err(|http_error| Error::new_from_message(&http_error.to_string()))?;

		let http_response_content = http_response.text()
			.await
			.map_err(|http_content_error| Error::new_from_message(&http_content_error.to_string()))?;

		// See https://docs.bunny.net/reference/bunnynet-api-overview -> Errors
		// If this parses then an error occured even the http request was successful
		let response_error_result: Result<Error, serde_json::Error> = serde_json::from_str(&http_response_content);
		if let Ok(response_error) = response_error_result {
			return Err(response_error);
		}
		// Attempt parse pagination. This is not present on all endpoints, but it is,
		// for example, on https://api.bunny.net/apikey.
		// This handles parsing the information, but it is up to the specific handler
		// to actually use the data
		let response_page_meta_result: Result<BunnyCDNPageMeta, serde_json::Error> = serde_json::from_str(&http_response_content);
		let page_meta = match response_page_meta_result {
			Ok(response_page_meta) => response_page_meta,
			Err(_) => BunnyCDNPageMeta::new(),
		};
		let response = BunnyCDNGetResponse{
			raw_data: http_response_content,
			page_meta,
		};
		return Ok(response);
	}
}

#[cfg(test)]
mod client_tests {
	use super::BunnyCDNClient;

	#[test]
	fn test_client_from_env() {
		let _ = dotenvy::dotenv();
		let _client_result = BunnyCDNClient::new_from_env();
	}

}