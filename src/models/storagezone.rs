use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use super::{serialize_datetime, deserialize_datetime, pullzone::PullZone};


#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum StorageZoneTier {
	Standard,
	Edge,
}

impl Default for StorageZoneTier {

	fn default() -> Self {
		return StorageZoneTier::Standard;
	}
}

// See https://docs.bunny.net/reference/storagezonepublic_index for further documentation
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct StorageZone {
	// The ID of the storage zone
	pub id: i64,
	// The ID of the user that owns the storage zone
	pub user_id: String,
	// The name of the storage zone
	pub name: String,
	// The API access key or FTP password
	pub password: String,
	// The date when the zone was last modified
	#[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
	pub date_modified: DateTime<Utc>,
	// Determines if the zone was deleted or not
	pub deleted: bool,
	// The total amount of storage used by this zone
	pub storage_used: i64,
	// The total number of files stored by this zone
	pub files_stored: i64,
	// The main region used by the storage zone
	pub region: String,
	// The replication regions enabled for this storage zone
	pub replication_regions: Vec<String>,
	pub pull_zones: Option<Vec<PullZone>>,
	// The read-only API access key or FTP password
	pub read_only_password: Option<String>,
	// Determines if the storage zone will rewrite 404 status codes to 200 status codes
	pub rewrite_404_to_200: bool,
	// The custom 404 error path that will be returned in case of a missing file
	pub custom_404_file_path: Option<String>,
	// Determines the storage hostname for this zone
	pub storage_host_name: Option<String>,
	pub zone_tier: StorageZoneTier,
	// Determines if the storage zone is currently enabling a new replication region
	pub replication_change_in_progress: bool,
	// The custom price override for this zone
	pub price_override: f64,
	// The Storage Zone specific pricing discount
	pub discount: i32,
}