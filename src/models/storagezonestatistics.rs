use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct StorageZoneStatistics {
	pub storage_used_chart: BTreeMap<String, u64>,
	pub file_count_chart: BTreeMap<String, u64>,
}