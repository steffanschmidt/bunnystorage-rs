use serde_json::Value;

use crate::{errors::Error, models::file::File};

use super::BunnyCDNClient;


impl BunnyCDNClient {
	
	/*
		The directory is relative to the root directory of the storage name
		See https://docs.bunny.net/reference/get_-storagezonename-path- for documentation
	 */
	pub async fn get_files(&self, directory: &str) -> Result<Vec<File>, Error> {
		let mut used_directory = directory.trim();
		if used_directory.is_empty() {
			used_directory = "/";
		}
		let endpoint_url = self.config.endpoint.url();
		let files_url = format!(
			"{}/{}/{}/",
			endpoint_url,
			self.config.storage_zone_name,
			used_directory
		);
		println!("{} - AccessKey: {}", files_url, self.config.read_password);
		let files_response = self.get(
			&files_url,
			&self.config.read_password,
			None
		).await?;
		let files_value_result: Value = serde_json::from_str(&files_response.raw_data)
			.map_err(|files_value_error| Error::new_from_message(&files_value_error.to_string()))?;

		let files_array_opt = files_value_result.as_array();
		if files_array_opt.is_none() {
			return Err(Error::new_from_message("Invalid Files Array"));
		}
		let mut files: Vec<File> = Vec::new();
		let files_array = files_array_opt.unwrap();
		for file_value in files_array.iter() {
			let file_content: String = file_value.to_string();
			let file_result: Result<File, Error> = serde_json::from_value(file_value.to_owned())
				.map_err(|file_parse_error| Error::new_from_message(&file_parse_error.to_string()));

			if let Err(file_error) = file_result {
				println!("File Content: {} Error: {}\n", file_content, file_error.to_string());
				break;
			}
			files.push(file_result.unwrap());
		}
		return Ok(files);
	}
}

#[cfg(test)]
mod files_test {
	use super::*;

	#[tokio::test]
	async fn test_get_files() {
		let client_result = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_file_directory = "/Test";
		let files_result = client_result.unwrap().get_files(test_file_directory).await;
		if let Err(files_error) = &files_result {
			println!("Failed Retrieving Files - Error: {}", files_error.to_string());
		}
		assert!(files_result.is_ok());
		println!("Files Count: {}", files_result.unwrap().len());
	}
}