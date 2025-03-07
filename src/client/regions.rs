use crate::{errors::Error, models::region::Region};

use super::{BunnyCDNClient, BUNNY_STORAGE_API_ROOT};


impl BunnyCDNClient {

	pub async fn get_regions(&self) -> Result<Vec<Region>, Error> {
		let regions_url = format!("{}/region", BUNNY_STORAGE_API_ROOT);
		let regions_response = self.get(
			&regions_url, 
			&self.config.api_key, 
			None
		).await?;
		return serde_json::from_str(&regions_response.raw_data)
			.map_err(|parse_error| Error::new_from_message(&parse_error.to_string()));
	}
}

#[cfg(test)]
mod regions_test {
	use super::*;

	#[tokio::test]
	async fn test_get_regions() {
		let client_result = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let regions_result = client_result.unwrap().get_regions().await;
		if let Err(regions_error) = &regions_result {
			println!("Failed Retrieving Regions - Error {}", regions_error.to_string());
		}
		assert!(regions_result.is_ok());
	}
}