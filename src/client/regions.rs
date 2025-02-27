use crate::{errors::Error, models::region::Region};

use super::{Client, BUNNY_STORAGE_API_ROOT};


impl Client {

	pub async fn get_regions(&self) -> Result<Vec<Region>, Error> {
		let regions_url = format!("{}/region", BUNNY_STORAGE_API_ROOT);
		let regions_content = self.get(&regions_url, None)
			.await?;

		let regions: Vec<Region> = serde_json::from_str(&regions_content)
			.map_err(|regions_parse_err| Error::new_from_message(&regions_parse_err.to_string()))?;

		return Ok(regions);
	}
}

#[cfg(test)]
mod regions_test {
	use super::*;



	#[tokio::test]
	async fn test_get_regions() {
		let client = Client::new_from_env();
		let regions_result = client.get_regions().await;
		if let Err(regions_error) = &regions_result {
			println!("Failed Retrieving Regions - Error {}", regions_error.message);
		}
		assert!(regions_result.is_ok());

	}
}