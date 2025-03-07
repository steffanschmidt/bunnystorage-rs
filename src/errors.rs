use std::fmt::Display;

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

impl Display for Error {

	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut displayed_error_message_buffer: Vec<String> = Vec::new();
		if !self.error_key.is_empty() {
			displayed_error_message_buffer.push(format!("Key: {}, ", self.error_key));
		}
		if !self.field.is_empty() {
			displayed_error_message_buffer.push(format!("Field: {}", self.field));
		}
		if !self.message.is_empty() {
			displayed_error_message_buffer.push(format!("Message: {}", self.message));
		}
		let mut displayed_error_message: String = String::from("Unknown");
		if !displayed_error_message_buffer.is_empty() {
			displayed_error_message = displayed_error_message_buffer.join(", ");
		}
		return write!(f, "{}", displayed_error_message);
	}
}