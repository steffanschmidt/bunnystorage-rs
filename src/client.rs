use std::collections::HashMap;

use crate::errors::Error;

pub mod statistics;
pub mod regions;

const BUNNY_STORAGE_API_ROOT: &str = "https://api.bunny.net";
const ENV_BUNNY_STORAGE_API_KEY_NAME: &str = "BUNNYSTORAGE_API_KEY";


pub struct ClientConfig {
	api_key: String,
}

impl ClientConfig {
	pub fn new(api_key: String) -> ClientConfig {
		return ClientConfig {
			api_key,
		}
	}

	pub fn new_from_env() -> ClientConfig {
		let initialize_env_result = dotenvy::dotenv();
		if let Err(initialize_env_error) = initialize_env_result {
			panic!("Failed Initizling Environemnt - Error {}", initialize_env_error.to_string());
		}
		let api_key_result = std::env::var(ENV_BUNNY_STORAGE_API_KEY_NAME);
		if let Err(api_key_error) = api_key_result {
			panic!("Missing API key - Error {}", api_key_error.to_string());
		}
		let client_config: ClientConfig = ClientConfig{
			api_key: api_key_result.unwrap(),
		};
		return client_config;
	}
}

#[allow(dead_code)]
pub struct Client {
	config: ClientConfig,
	http_client: reqwest::Client,
}

impl Client {
	pub fn new(config: ClientConfig) -> Client {
		let client = Client{
			config,
			http_client: reqwest::Client::new(),
		};
		return client;
	}

	pub fn new_from_env() -> Client {
		let client_config = ClientConfig::new_from_env();
		return Self::new(client_config);
	}

	async fn get(&self, url: &str, params: Option<&HashMap<&str, String>>) -> Result<String, Error> {
		let http_response = self.http_client.get(url)
			.header("AccessKey", &self.config.api_key)
			.query(&params)
			.send()
			.await
			.map_err(|http_error| Error::new_from_message(&http_error.to_string()))?;

		let http_response_content = http_response.text()
			.await
			.map_err(|http_content_error| Error::new_from_message(&http_content_error.to_string()))?;
		
		return Ok(http_response_content);
	}
}

#[cfg(test)]
mod client_tests {
	use super::Client;

	#[test]
	fn test_client_from_env() {
		let _ = dotenvy::dotenv();
		let _client_result = Client::new_from_env();
	}

}