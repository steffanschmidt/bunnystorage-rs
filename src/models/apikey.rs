use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct APIKey {
	pub id: i64,
	pub key: String,
	pub roles: Vec<String>,
}