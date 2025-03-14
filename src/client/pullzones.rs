use std::collections::HashMap;

use crate::{errors::Error, models::pullzone::PullZone};

use super::{BunnyCDNClient, BunnyCDNPageParameters, BUNNY_STORAGE_API_ROOT};

const PULL_ZONE_INCLUDE_CERTIFICATE_NAME: &str = "includeCertificate";

pub struct PullZonesParameters {
	pub search: Option<String>,
	pub include_certificate: Option<bool>,
}

impl BunnyCDNClient {

	pub async fn get_pull_zones(&self, params: Option<&PullZonesParameters>, page_params: Option<&BunnyCDNPageParameters>) -> Result<Vec<PullZone>, Error> {
		let mut pull_zones_parameters = HashMap::<&str, String>::new();
		self.add_page_parameters(&mut pull_zones_parameters, page_params);
		if let Some(provided_params) = params {
			if let Some(search) = &provided_params.search {
				pull_zones_parameters.insert("search", search.to_string());
			}
			if let Some(include_certificate) = &provided_params.include_certificate {
				pull_zones_parameters.insert(PULL_ZONE_INCLUDE_CERTIFICATE_NAME, include_certificate.to_string());
			}
		}
		let pull_zones_url = format!(
			"{}/pullzone",
			BUNNY_STORAGE_API_ROOT,
		);
		let pull_zones_response = self.get(
			&pull_zones_url,
			&self.config.api_key,
			Some(&pull_zones_parameters),
		).await?;
		let pull_zones: Vec<PullZone> = serde_json::from_str(&pull_zones_response.body)
			.map_err(|deserialize_pull_error| Error::new_from_message(&deserialize_pull_error.to_string()))?;
		return Ok(pull_zones);
	}

	pub async fn get_pull_zone(&self, id: i64, include_certificate: Option<bool>) -> Result<PullZone, Error> {
		let mut pull_zone_parameters = HashMap::<&str, String>::new();
		if let Some(provided_include_certificate) = include_certificate {
			pull_zone_parameters.insert(PULL_ZONE_INCLUDE_CERTIFICATE_NAME, provided_include_certificate.to_string());
		}
		let pull_zone_url = format!(
			"{}/pullzone/{}",
			BUNNY_STORAGE_API_ROOT,
			id,
		);
		let pull_zone_response = self.get(
			&pull_zone_url,
			&self.config.api_key,
			Some(&pull_zone_parameters)
		).await?;
		let pull_zone: PullZone = serde_json::from_str(&pull_zone_response.body)
			.map_err(|deserialize_error| Error::new_from_message(&deserialize_error.to_string()))?;
		return Ok(pull_zone);
	}

}

#[cfg(test)]
mod pull_zone_test {
	use crate::environment::get_i64_from_env;
	use super::*;

	#[tokio::test]
	async fn test_get_pull_zones() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let get_pull_zones_result = client_result
			.unwrap()
			.get_pull_zones(
				None,
				None
			).await;
		if let Err(get_pull_zones_error) = &get_pull_zones_result {
			println!("Failed Retrieving Pull Zones - Error: {}", get_pull_zones_error.to_string());
		}
		assert!(get_pull_zones_result.is_ok());
	}

	#[tokio::test]
	async fn test_get_pull_zone() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_id_result = get_i64_from_env("BUNNYSTORAGE_PULL_ZONE_TEST_ID");
		assert!(test_id_result.is_ok());
		let get_pull_zones_result = client_result
			.unwrap()
			.get_pull_zone(
				test_id_result.unwrap(),
				None
			).await;
		if let Err(get_pull_zones_error) = &get_pull_zones_result {
			println!("Failed Retrieving Pull Zone - Error: {}", get_pull_zones_error.to_string());
		}
		assert!(get_pull_zones_result.is_ok());
	}
	
}