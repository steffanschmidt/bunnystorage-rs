use std::collections::HashMap;

use chrono::{DateTime, Utc};
use crate::{errors::Error, models::statistics::Statistics};
use super::{BunnyCDNClient, BUNNY_STORAGE_API_ROOT};

// See https://api.bunny.net/statistics
#[derive(Debug)]
pub struct StatisticsParameters {
	// The start date of the statistics. If no value is passed, the last 30 days will be returned.
	pub date_from: Option<DateTime<Utc>>,
	// The end date of the statistics. If no value is passed, the last 30 days will be returned.
	pub date_to: Option<DateTime<Utc>>,
	// If set, the statistics will be only returned for the given Pull Zone
	pub pull_zone: Option<i64>,
	// If set, the statistics will be only returned for the given region ID
	pub server_zone_id: Option<i64>,
	// If set, the respose will contain the non-2xx response
	pub load_errors: Option<bool>,
	// If true, the statistics data will be returned in hourly groupping
	pub hourly: Option<bool>,
}

fn prepare_statistics_params(optional_params: Option<&StatisticsParameters>) -> HashMap<&str, String> {
	let mut params_map: HashMap<&str, String> = HashMap::new();
	match optional_params {
		None => return params_map,
		Some(params) => {
			if let Some(date_from) = params.date_from {
				params_map.insert("dateFrom", date_from.to_rfc3339());
			}
			if let Some(date_to) = params.date_to {
				params_map.insert("dateTo", date_to.to_rfc3339());
			}
			if let Some(pull_zone) = params.pull_zone {
				params_map.insert("pullZone", pull_zone.to_string());
			}
			if let Some(server_zone_id) = params.server_zone_id {
				params_map.insert("serverZoneId", server_zone_id.to_string());
			}
			if let Some(load_errors) = params.load_errors {
				params_map.insert("loadErrors", load_errors.to_string());
			}
			if let Some(hourly) = params.hourly {
				params_map.insert("hourly", hourly.to_string());
			}
			return params_map;
		}
	}
}

impl BunnyCDNClient {
	
	pub async fn get_statistics(&self, params: Option<&StatisticsParameters>) -> Result<Statistics, Error> {
		let statistics_url: String = format!("{}/statistics", BUNNY_STORAGE_API_ROOT.to_string());
		let params_map = prepare_statistics_params(params);
		let statistics_response = self.get(
			&statistics_url,
			&self.config.api_key,
			Some(&params_map)).await?;
		return serde_json::from_str(&statistics_response.raw_data)
			.map_err(|parse_error| Error::new_from_message(&parse_error.to_string()));
	}
}

#[cfg(test)]
mod statistics_tests {
	use tokio;
	use super::*;

	#[tokio::test]
	async fn test_get_statistics() {
		let client_result = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let statistics_result = client_result.unwrap().get_statistics(None).await;
		if let Err(statistics_error) = &statistics_result {
			println!("Failed Getting Statistic - Error {}", statistics_error.to_string());
		}
		assert!(statistics_result.is_ok());
	}
}