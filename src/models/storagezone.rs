use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::{serialize_datetime, deserialize_datetime, pullzone::PullZone};


// See https://docs.bunny.net/reference/storagezonepublic_index for further documentation
#[derive(Debug, Serialize, Deserialize)]
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
	#[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
	pub date_modified: DateTime<Utc>,
	pub deleted: bool,
	pub storage_used: i64,
	pub files_stored: i64,
	pub region: String,
	pub replication_regions: Vec<String>,
	pub pull_zones: Vec<PullZone>,
}