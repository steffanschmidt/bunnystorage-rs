use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{client::{BunnyCDNDataOptions, ContentType, CONTENT_TYPE_HEADER_NAME}, errors::Error, models::{storagezone::{StorageZone, StorageZoneTier}, storagezonestatistics::StorageZoneStatistics}};

use super::{BunnyCDNClient, BunnyCDNPageParameters, BUNNY_STORAGE_API_ROOT};

pub struct GetStorageZoneParameters {
	pub include_deleted: Option<bool>,
	pub search: Option<String>
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct AddStorageZoneCommonParameters {
	#[serde(skip_serializing_if = "Option::is_none")]
	replication_regions: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	origin_url: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct AddStorageZoneParameters {
	name: String,
	region: String,
	zone_tier: StorageZoneTier,
	#[serde(skip_serializing_if = "Option::is_none")]
	common: Option<AddStorageZoneCommonParameters>,
}

#[derive(Debug, Serialize)]
pub struct UpdateStorageZoneParameters {
	#[serde(skip_serializing_if = "Option::is_none")]
	custom404_file_path: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	rewrite404_to200: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	common: Option<AddStorageZoneCommonParameters>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
struct CheckStorageZoneAvailability {
	available: bool,
}

pub struct StorageZoneStatisticsParameters {
	// The start date of the statistics. If no value is passed, the last 30 days will be returned.
	pub date_from: Option<DateTime<Utc>>,
	// The end date of the statistics. If no value is passed, the last 30 days will be returned.
	pub date_to: Option<DateTime<Utc>>
}

impl BunnyCDNClient {

	fn get_storage_zone_root_url(&self) -> String {
		return format!(
			"{}/storagezone",
			BUNNY_STORAGE_API_ROOT,
		);
	}

	pub async fn get_storage_zones(&self, params: Option<&GetStorageZoneParameters>, page_params: Option<&BunnyCDNPageParameters>) -> Result<Vec<StorageZone>, Error> {
		let mut storage_zone_params = HashMap::<&str, String>::new();
		self.add_page_parameters(&mut storage_zone_params, page_params);
		if let Some(provided_params) = params {
			if let Some(included_delete) = &provided_params.include_deleted {
				storage_zone_params.insert("includeDeleted", included_delete.to_string());
			}
			if let Some(search) = &provided_params.search {
				storage_zone_params.insert("search", search.to_string());
			}
		}
		let get_storage_zones_url = self.get_storage_zone_root_url();
		let storage_zones_response = self.get(
			&get_storage_zones_url,
			&self.config.api_key,
			Some(&storage_zone_params),
		).await
		.map_err(|get_storage_zones_error| Error::new_from_message(&get_storage_zones_error.to_string()))?;

		let storage_zones: Vec<StorageZone> = serde_json::from_str(&storage_zones_response.body)
			.map_err(|deserialize_error| Error::new_from_message(&deserialize_error.to_string()))?;

		return Ok(storage_zones);
	}

	/// Retrieve a single storage zone. It is likely that the id must be > 0.
	/// 
	/// However this is not controlled in this end, since it is better to leave the validation
	/// to Bunnystorage in case this assumption is not correct
	/// 
	/// # Examples
	/// ```
	///	use bunnystorage_rs::client::BunnyCDNClient;
	///	use bunnystorage_rs::errors::Error;
	///	async fn my_test() -> Result<(), Error> {
	///		let client: BunnyCDNClient = BunnyCDNClient::new_from_env()?;
	///		let id: i64 = 1234;
	///		let storage_zone_result = client.get_storage_zone(id).await;
	///		if let Err(storage_zone_err) = storage_zone_result {
	///			println!("Oh No {}", storage_zone_err.to_string());
	///		}
	///		// Do something with the Storage Zone
	///		return Ok(());
	///	}
	/// ```
	pub async fn get_storage_zone(&self, id: i64) -> Result<StorageZone, Error> {
		let get_storage_zone_url = format!(
			"{}/{}",
			self.get_storage_zone_root_url(),
			id,
		);
		let storage_zone_response = self.get(
			&get_storage_zone_url,
			&self.config.api_key,
			None,
		).await
		.map_err(|storage_zone_error| Error::new_from_message(&storage_zone_error.to_string()))?;
		let storage_zone: StorageZone = serde_json::from_str(&storage_zone_response.body)
			.map_err(|deserialize_error| Error::new_from_message(&deserialize_error.to_string()))?;

		return Ok(storage_zone);
	}

	/// Checks to see if the storage zone name is available.
	/// Both active and deleted names are included in this search on Bunnystorage
	pub async fn check_storage_zone_availability(&self, name: &str) -> Result<bool, Error> {
		let used_name = name.trim();
		if used_name.is_empty() {
			return Err(Error::new_from_message("Invalid Storage Zone Name. Must not be empty"));
		}
		let check_storage_zone_availability_url = format!(
			"{}/checkavailability",
			self.get_storage_zone_root_url(),
		);
		let mut check_storage_zone_availability_map = HashMap::<&str, &str>::new();
		check_storage_zone_availability_map.insert("Name", used_name);
		let mut check_storage_zone_availability_headers: HashMap<String, String> = HashMap::new();
		check_storage_zone_availability_headers.insert(CONTENT_TYPE_HEADER_NAME.to_string(), ContentType::ApplicationJson.name().to_string());
		let check_storage_zone_availability_options = BunnyCDNDataOptions{
			headers: Some(check_storage_zone_availability_headers),
		};
		let check_storage_zone_availability_data = serde_json::to_string(&check_storage_zone_availability_map)
			.map_err(|serialize_error| Error::new_from_message(&serialize_error.to_string()))?;

		let check_storage_zone_availability_response = self.post(
			&check_storage_zone_availability_url,
			&self.config.api_key,
			check_storage_zone_availability_data,
			Some(&check_storage_zone_availability_options),
		).await
		.map_err(|check_storage_zone_availability_error| Error::new_from_message(&check_storage_zone_availability_error.to_string()))?;
		
		let storage_zone_availabile: CheckStorageZoneAvailability = serde_json::from_str(&check_storage_zone_availability_response.body)
			.map_err(|deseriliaze_error| Error::new_from_message(&deseriliaze_error.to_string()))?;

		return Ok(storage_zone_availabile.available);
	}

	/// This function firstly check if the storage zone is available
	/// It is does, then it will attempt to retrieve the storage zone by calling
	/// get_storage_zones and attempt to find and return it
	/// Parameters:
	/// 
	/// * name - the storage zone name to find
	/// * included_deleted - include deleted storage zones when searching
	pub async fn attempt_find_storage_zone(&self, name: &str, include_deleted: Option<bool>) -> Result<Option<StorageZone>, Error> {
		let storage_zone_available = self.check_storage_zone_availability(&name).await?;
		if !storage_zone_available {
			let used_include_deleted = match include_deleted {
				Some(include) => include,
				None => true,
			};
			let storage_zones_params = GetStorageZoneParameters{
				include_deleted: Some(used_include_deleted),
				search: Some(name.to_string()),
			};
			let storage_zones = self.get_storage_zones(
				Some(&storage_zones_params),
				None,
			).await?;
			let storage_zone_name_to_found = name.to_lowercase().trim().to_string();
			for storage_zone in storage_zones.iter() {
				let found_storage_zone_name = storage_zone.name.to_lowercase().trim().to_string();
				if storage_zone_name_to_found == found_storage_zone_name {
					return Ok(Some(storage_zone.clone()));
				}
			}
		}
		return Ok(None);
	}

	/// Attempt to add a storage zone regions.
	/// 
	/// You may obtain the list of regions by calling get_regions on this client.
	/// This is a long list and it may be expanded upon by Bunnystorage in the future.
	/// Therefore it is not included as an enum.
	/// 
	/// Note:
	/// When one attempts to add a storage zone which already exists, then an error is returned.
	/// This zone can both be active and deleted. The error key name for this is 'storagezone.name_taken'
	/// If one wishes to avoid this, then use instead add_storage_zone_exist_ok or call 
	/// or check_storage_zone_availability before calling this.
	/// 
	/// Alternatively use the wrapper function add_storage_zone_exists_ok if you want the storage
	/// zone returned
	pub async fn add_storage_zone(&self, params: &AddStorageZoneParameters) -> Result<StorageZone, Error> {
		let used_name = params.name.trim();
		if used_name.is_empty() {
			return Err(Error::new_from_message("Invalid Storage Zone Name. Must not be empty"));
		}
		let used_region =  params.region.trim();
		if used_region.is_empty() {
			return Err(Error::new_from_message("Invalid Storage Zone Region. Must not be empty"));
		}
		let serialized_data: String = serde_json::to_string(&params)
			.map_err(|serialized_error| Error::new_from_message(&serialized_error.to_string()))?;

		let add_storage_zone_url = self.get_storage_zone_root_url();
		let mut add_storage_zone_headers: HashMap<String, String> = HashMap::new();
		add_storage_zone_headers.insert(CONTENT_TYPE_HEADER_NAME.to_string(), ContentType::ApplicationJson.name().to_string());
		let add_storage_zone_options = BunnyCDNDataOptions{
			headers: Some(add_storage_zone_headers),
		};
		let add_storage_zone_response = self.post(
			&add_storage_zone_url,
			&self.config.api_key,
			serialized_data,
			Some(&add_storage_zone_options),
		).await
		.map_err(|add_storage_zone_error| Error::new_from_message(&add_storage_zone_error.to_string()))?;

		let storage_zone: StorageZone = serde_json::from_str(&add_storage_zone_response.body)
			.map_err(|deserialize_error| Error::new_from_message(&deserialize_error.to_string()))?;

		return Ok(storage_zone);
	}
	
	/// Unlike add_storage_zone this function 
	/// Utilities attempt_find_storage_zone and add_storage_zone
	pub async fn add_storage_zone_exists_ok(&self, params: &AddStorageZoneParameters, include_deleted: Option<bool>) -> Result<StorageZone, Error> {
		let found_storage_zone_opt = self.attempt_find_storage_zone(
			&params.name,
			include_deleted
		).await?;
		if let Some(found_storage_zone) = found_storage_zone_opt {
			return Ok(found_storage_zone);
		}
		return self.add_storage_zone(params).await;
	}

	// pub async fn update_storage_zone(&self, id: i64, params: &UpdateStorageZoneParameters) -> Result<StorageZone, Error> {
		
	// }

	/// Attempts to delete a storage zone. If the storage has already been deleted previously
	/// this will return an error. If you want to avoid this, then call delete_storage_zone_check
	pub async fn delete_storage_zone(&self, id: i64) -> Result<(), Error> {
		let delete_storage_zone_url = format!(
			"{}/{}",
			self.get_storage_zone_root_url(),
			id,
		);
		return self.delete(
			&delete_storage_zone_url,
			&self.config.api_key,
		).await;
	}

	pub async fn get_storage_zone_statistics(&self, id: i64, params: Option<&StorageZoneStatisticsParameters>) -> Result<StorageZoneStatistics, Error> {
		let storage_zone_statistics_url = format!(
			"{}/{}/statistics",
			self.get_storage_zone_root_url(),
			id,
		);
		let mut storage_zone_statistics_parameters = HashMap::<&str, String>::new();
		if let Some(provided_params) = params {
			if let Some(date_from) = provided_params.date_from {
				storage_zone_statistics_parameters.insert("dateFrom", date_from.to_rfc3339());
			}
			if let Some(date_to) = provided_params.date_to {
				storage_zone_statistics_parameters.insert("dateFrom", date_to.to_rfc3339());
			}
		}
		let storage_zone_statistics_response = self.get(
			&storage_zone_statistics_url,
			&self.config.api_key,
			Some(&storage_zone_statistics_parameters),
		).await?;
		let storage_zone_statistics: StorageZoneStatistics = serde_json::from_str(&storage_zone_statistics_response.body)
			.map_err(|deserialize_error| Error::new_from_message(&deserialize_error.to_string()))?;

		return Ok(storage_zone_statistics);
	}

}

#[cfg(test)]
mod storage_zone_test {
	use crate::environment::get_i64_from_env;

use super::*;

	#[tokio::test]
	async fn test_get_storage_zones() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let storage_zones_result = client_result
			.unwrap()
			.get_storage_zones(
				None,
				None,
			)
			.await;
		if let Err(storage_zones_error) = &storage_zones_result {
			println!("Failed Retrieving Storage Zones - Error {}", storage_zones_error.to_string());
		}
		assert!(storage_zones_result.is_ok());
	}

	#[tokio::test]
	async fn test_get_storage_zone() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_storage_zone_id_result = get_i64_from_env("BUNNYSTORAGE_STORAGE_ZONE_TEST_ID");
		if let Err(_) = test_storage_zone_id_result {
			println!("Cannot test Get Storage Zone - Missing BUNNYSTORAGE_STORAGE_ZONE_TEST_ID in environment or ");
			return;
		}
		let get_storage_zone_result = client_result
			.unwrap()
			.get_storage_zone(
				test_storage_zone_id_result.unwrap(),
			)
			.await;
		if let Err(storage_zones_error) = &get_storage_zone_result {
			println!("Failed Retrieving Storage Zones - Error {}", storage_zones_error.to_string());
		}
		assert!(get_storage_zone_result.is_ok());
	}

	#[tokio::test]
	async fn test_add_storage_zone() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let add_storage_zone_params = AddStorageZoneParameters{
			name: "AddTestStorageZone".to_string(),
			region: "DE".to_string(),
			zone_tier: StorageZoneTier::Standard,
			common: None,
		};
		let add_storage_zone_result = client_result
			.unwrap()
			.add_storage_zone_exists_ok(
				&add_storage_zone_params,
				Some(true),
			)
			.await;
		if let Err(storage_zones_error) = &add_storage_zone_result {
			println!("Failed Retrieving Storage Zones - Error {}", storage_zones_error.to_string());
		}		
		assert!(&add_storage_zone_result.is_ok());
	}

	#[tokio::test]
	async fn test_get_storage_zone_statistics() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_storage_zone_id_result = get_i64_from_env("BUNNYSTORAGE_STORAGE_ZONE_TEST_ID");
		if let Err(_) = test_storage_zone_id_result {
			println!("Cannot test Get Storage Zone - Missing BUNNYSTORAGE_STORAGE_ZONE_TEST_ID in environment or ");
			return;
		}
		let storage_zone_statistics_params = StorageZoneStatisticsParameters{
			date_from: None,
			date_to: None,
		};
		let test_storage_zone_statistics_result = client_result
			.unwrap()
			.get_storage_zone_statistics(
				test_storage_zone_id_result.unwrap(),
				Some(&storage_zone_statistics_params)
			)
			.await;
		
		if let Err(test_storage_zone_statistics_error) = &test_storage_zone_statistics_result {
			println!("Failed Retrieving Storage Zone Statistics - {}", test_storage_zone_statistics_error.to_string());
		}
		assert!(test_storage_zone_statistics_result.is_ok());
	}

}