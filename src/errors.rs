use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Error {
	pub error_key: String,
	pub field: String,
	pub message: String,
}

impl Error {

	pub fn new_from_message(message: &str) -> Error {
		return Error{
			error_key: "".to_string(),
			field: "".to_string(),
			message: message.to_string(),
		}
	}

}
