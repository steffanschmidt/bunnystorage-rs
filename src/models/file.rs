use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::{serialize_datetime, deserialize_datetime};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct File {
	guid: String,
	storage_zone_name: String,
	path: String,
	object_name: String,
	length: u64,
	#[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
	last_changed: DateTime<Utc>,
	server_id: u32,
	array_number: u32,
	is_directory: bool,
	user_id: String,
	content_type: String,
	#[serde(serialize_with = "serialize_datetime", deserialize_with = "deserialize_datetime")]
	date_created: DateTime<Utc>,
	storage_zone_id: u32,
	checksum: Option<String>,
	replicated_zones: Option<String>,
}