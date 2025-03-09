use std::collections::HashMap;

use reqwest::{Body, StatusCode};
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

const ACCESS_KEY_HEADER_NAME: &str = "AccessKey";
const CONTENT_TYPE_HEADER_NAME: &str = "Content-Type";

// Headers
pub enum ContentType {
	ApplicationJson,
	ApplicationOctetStream,
}

impl ContentType {
	pub fn name(&self) -> &str {
		match self {
			ContentType::ApplicationJson => "application/json",
			ContentType::ApplicationOctetStream => "application/octet-stream",
		}
	}
}

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

pub struct BunnyCDNDataOptions {
	pub headers: Option<HashMap<String, String>>,
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

	fn check_write_password_ok(&self) -> Result<(), Error> {
		if let Some(write_password) = &self.config.write_password {
			if write_password.is_empty() {
				return Err(Error::new_from_message("Invalid Write Password"));
			}
			return Ok(());
		}
		return Err(Error::new_from_message("No Write Password"));
	}

	// See https://docs.bunny.net/reference/bunnynet-api-overview -> Errors
	// If this parses then an error occured even the http request was successful
	fn attempt_parse_request_error(&self, response_content: &str) -> Result<(), Error> {
		let response_error_result: Result<Error, serde_json::Error> = serde_json::from_str(&response_content);
		if let Ok(response_error) = response_error_result {
			return Err(response_error);
		}
		return Ok(());
	}

	async fn get(&self, url: &str, access_key: &str, params: Option<&HashMap<&str, String>>) -> Result<BunnyCDNGetResponse, Error> {
		let http_get_request = self.http_client.get(url)
			.header(ACCESS_KEY_HEADER_NAME, access_key)
			.query(&params);

		let http_get_response = http_get_request.send()
			.await
			.map_err(|http_get_error| Error::new_from_message(&http_get_error.to_string()))?
			.error_for_status()
			.map_err(|http_get_request_error| Error::new_from_message(&http_get_request_error.to_string()))?;

		if http_get_response.status() != StatusCode::OK {
			return Err(Error::new_from_message(&format!("Failed Get Request - Code {}", http_get_response.status())))
		}
		let http_get_response_content = http_get_response.text()
			.await
			.map_err(|http_get_content_error| Error::new_from_message(&http_get_content_error.to_string()))?;

		self.attempt_parse_request_error(&http_get_response_content)?;
		// Attempt parse pagination. This is not present on all endpoints, but it is,
		// for example, on https://api.bunny.net/apikey.
		// This handles parsing the information, but it is up to the specific handler
		// to actually use the data
		let response_page_meta_result: Result<BunnyCDNPageMeta, serde_json::Error> = serde_json::from_str(&http_get_response_content);
		let page_meta = match response_page_meta_result {
			Ok(response_page_meta) => response_page_meta,
			Err(_) => BunnyCDNPageMeta::new(),
		};
		let response = BunnyCDNGetResponse{
			raw_data: http_get_response_content,
			page_meta,
		};
		return Ok(response);
	}

	async fn put<T: Into<Body>>(&self, url: &str, access_key: &str, data: T, options: Option<&BunnyCDNDataOptions>) -> Result<(), Error> {
		// Setup the Request
		let mut http_put_request = self.http_client.put(url)
			.body(data);

		if let Some(provided_options) = options {
			if let Some(headers) = &provided_options.headers {
				for (header_name, header_value) in headers.iter() {
					http_put_request = http_put_request.header(header_name, header_value);
				}
			}
		}
		http_put_request = http_put_request.header(ACCESS_KEY_HEADER_NAME, access_key);
		// Perform the Request
		let http_put_response = http_put_request.send()
			.await
			.map_err(|http_put_response_error| Error::new_from_message(&http_put_response_error.to_string()))?
			.error_for_status()
			.map_err(|http_put_request_error| Error::new_from_message(&http_put_request_error.to_string()))?;

		let http_put_response_content = http_put_response.text()
			.await
			.map_err(|http_put_content_error| Error::new_from_message(&http_put_content_error.to_string()))?;
		
		self.attempt_parse_request_error(&http_put_response_content)?;
		return Ok(());
	}

	async fn delete(&self, url: &str, access_key: &str) -> Result<(), Error> {
		let http_delete_response = self.http_client.delete(url)
			.header(ACCESS_KEY_HEADER_NAME, access_key)
			.send()
			.await
			.map_err(|http_delete_error| Error::new_from_message(&http_delete_error.to_string()))?
			.error_for_status()
			.map_err(|http_delete_request_error| Error::new_from_message(&http_delete_request_error.to_string()))?;

		let http_delete_content = http_delete_response.text()
			.await
			.map_err(|http_delete_content_error| Error::new_from_message(&http_delete_content_error.to_string()))?;

		self.attempt_parse_request_error(&http_delete_content)?;
		return Ok(());
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