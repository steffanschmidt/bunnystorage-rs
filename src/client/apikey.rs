use std::collections::HashMap;

use crate::{models::apikey::APIKey, errors::Error};
use super::{BunnyCDNClient, BunnyCDNPageParameters, BUNNY_STORAGE_API_ROOT};

impl BunnyCDNClient {
	pub async fn get_api_keys(&self, params: Option<&BunnyCDNPageParameters>) -> Result<Vec<APIKey>, Error> {
		let mut params_map: HashMap<&str, String> = HashMap::new();
		self.add_page_parameters(&mut params_map, params);
		let api_keys_url = format!("{}/apikey", BUNNY_STORAGE_API_ROOT);
		let api_keys_response = self.get(
			&api_keys_url, 
			&self.config.api_key, 
			Some(&params_map)
		).await?;
		let mut api_keys = Vec::<APIKey>::new();
		for api_key_item in api_keys_response.page_meta.items.iter() {
			let api_key_result: Result<APIKey, serde_json::Error> = serde_json::from_value(api_key_item.to_owned());
			if let Err(api_key_error) = api_key_result {
				return Err(Error::new_from_message(&api_key_error.to_string()));
			}
			api_keys.push(api_key_result.unwrap());
		}
		return Ok(api_keys);
	}
}

#[cfg(test)]
mod api_keys_tests {
	use super::*;

	#[tokio::test] 
	async fn test_get_api_keys() {
		let client_result = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let api_keys_result = client_result.unwrap().get_api_keys(None).await;
		if let Err(api_keys_error) = &api_keys_result {
			println!("Failed Retrieving API Keys - Error: {}", api_keys_error.to_string());
		}
		assert!(api_keys_result.is_ok());
	}
}