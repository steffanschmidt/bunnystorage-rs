use crate::errors::BunnyStorageError;

const ENV_BUNNY_STORAGE_API_KEY_NAME: &str = "BUNNYSTORAGE_API_KEY";

struct ClientConfig {
	api_key: String,
}

impl ClientConfig {

	pub fn new(api_key: String) -> ClientConfig {
		return ClientConfig {
			api_key,
		}
	}

	pub fn new_from_env() -> Result<ClientConfig, BunnyStorageError> {
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
		println!("API KEY: {}", client_config.api_key);
		return Ok(client_config);
	}

}
pub struct Client {
	config: ClientConfig,
}

impl Client {

	pub fn new(api_key: String) -> Result<Client, BunnyStorageError> {
		let client = Client{
			config: ClientConfig::new(api_key),
		};
		return Ok(client);
	}

	pub fn new_from_env() -> Result<Client, BunnyStorageError> {
		let client_config = ClientConfig::new_from_env()?;
		let client = Client{
			config: client_config,
		};
		return Ok(client);
	}

}

#[cfg(test)]
mod client_tests {

    use super::Client;

	#[test]
	fn test_client_from_env() {
		let _ = dotenvy::dotenv();
		let client_result = Client::new_from_env();
		assert!(client_result.is_ok());
	}

}