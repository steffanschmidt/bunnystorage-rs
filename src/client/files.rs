use std::{collections::HashMap, ffi::OsStr, fs, io::Read, path};

use serde_json::Value;

use crate::{errors::Error, models::file::File};

use super::{BunnyCDNClient, BunnyCDNDataOptions, ContentType, CONTENT_TYPE_HEADER_NAME};


impl BunnyCDNClient {

	fn get_files_root_url(&self) -> String {
		let endpoint_url = self.config.endpoint.url();
		let files_root_url = format!(
			"{}/{}",
			endpoint_url,
			self.config.storage_zone_name,
		);
		return files_root_url
	}

	/*
		The directory is relative to the root directory of the storage name
		See https://docs.bunny.net/reference/get_-storagezonename-path- for documentation
	 */
	pub async fn get_files(&self, directory: &str) -> Result<Vec<File>, Error> {
		let mut used_directory = directory.trim().to_string();
		if used_directory.starts_with("/") {
			used_directory.insert_str(0, "/");
		}
		let files_url = format!(
			"{}/{}/",
			self.get_files_root_url(),
			used_directory
		);
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
			let file_result: Result<File, Error> = serde_json::from_value(file_value.to_owned())
				.map_err(|file_parse_error| Error::new_from_message(&file_parse_error.to_string()));

			if let Err(_) = file_result {
				break;
			}
			files.push(file_result.unwrap());
		}
		return Ok(files);
	}
	
	fn validate_filepath(&self, filepath: &str) -> Result<String, Error> {
		let trimmed_filepath = filepath.trim().to_string();
		if trimmed_filepath.is_empty() {
			return Err(Error::new_from_message("Invalid filepath"));
		}
		return Ok(trimmed_filepath.to_string());
	}

	fn validate_source_filepath(&self, filepath: &str) -> Result<String, Error> {
		let validate_filepath = self.validate_filepath(filepath)?;
		let absolute_filepath_buffer = path::absolute(validate_filepath)
			.map_err(|absolute_filepath_buffer_error| Error::new_from_message(&absolute_filepath_buffer_error.to_string()))?;

		let file_exists = fs::exists(&absolute_filepath_buffer)
			.map_err(|file_exists_error| Error::new_from_message(&file_exists_error.to_string()))?;

		if !file_exists {
			return Err(Error::new_from_message("Invalid Filepath. File does not exist"));
		}
		let filepath_option = absolute_filepath_buffer.to_str();
		return match filepath_option {
			None => Err(Error::new_from_message("Failed to retrieve the absolute filepath")),
			Some(filepath) => Ok(filepath.to_string()),
		}
	}

	fn retrieve_filepath_extension(&self, filepath: &str) -> Result<String, Error> {
		let filepath_extension_opt = path::Path::new(filepath)
			.extension()
			.and_then(OsStr::to_str);
		
		match filepath_extension_opt {
			Some(filepath_extension) => Ok(filepath_extension.to_string()),
			None => Err(Error::new_from_message("Failed extract file extension")),
		}
	}

	fn prepare_target_filepath(&self, filepath: &str) -> Result<String, Error> {
		let mut validated_filepath: String = self.validate_filepath(filepath)?;
		if validated_filepath.starts_with(".") {
			validated_filepath.remove(0);
		}
		if validated_filepath.starts_with("/") {
			validated_filepath.remove(0);
		}
		return Ok(validated_filepath);
	}

	fn evaluate_target_filepath(&self, source_filepath: &str, target_filepath: Option<&str>) -> Result<String, Error> {
		return match target_filepath {
			None => self.prepare_target_filepath(source_filepath),
			Some(provided_target_filepath) => {
				let target_file_extension = self.retrieve_filepath_extension(provided_target_filepath)?;
				let source_file_extension = self.retrieve_filepath_extension(source_filepath)?;
				if target_file_extension != source_file_extension {
					return Err(Error::new_from_message("Invalid target file extension. Must match source"));
				}
				return self.prepare_target_filepath(provided_target_filepath);
			}
		};
	}

	/*
		If the target is not provided, then the source filepath will be used.
		For example say that a file is stored on /my/test/directory/file.json, then
		the target, if not provided, will be uploaded to that in Bunny Storage.
		However is a target is provided, then this will be used instead.
		The target must be filepath to allow for renaming a file and the
		file extensions must match.
	*/
	pub async fn upload_file(&self, source_filepath: &str, target_filepath: Option<&str>) -> Result<(), Error> {
		self.check_write_password_ok()?;
		let used_source_filepath: String = self.validate_source_filepath(source_filepath)?;
		let mut file_contents = Vec::new();
		let mut file = fs::File::open(used_source_filepath)
			.map_err(|open_file_error| Error::new_from_message(&open_file_error.to_string()))?;
		
		let read_file_result = file.read_to_end(&mut file_contents);
		if let Err(read_file_error) = read_file_result {
			return Err(Error::new_from_message(&format!("Failed Read File - {}", read_file_error.to_string())));
		}
		let used_target_filepath: String = self.evaluate_target_filepath(source_filepath, target_filepath)?;
		let upload_file_url: String = format!(
			"{}/{}",
			self.get_files_root_url(),
			used_target_filepath,
		);
		let mut upload_file_headers = HashMap::<String, String>::new();
		upload_file_headers.insert(CONTENT_TYPE_HEADER_NAME.to_string(), ContentType::ApplicationOctetStream.name().to_string());
		let upload_file_options = BunnyCDNDataOptions{
			headers: Some(upload_file_headers),
		};
		let write_password = self.config.write_password.clone().unwrap();
		let upload_file_result = self.put(
			&upload_file_url,
			&write_password,
			file_contents,
			Some(&upload_file_options),
		).await;
		return upload_file_result
	}

	// If the filepath is a directory on Bunny Storage, then the directory
	// will be deleted recursively. See
	// https://docs.bunny.net/reference/delete_-storagezonename-path-filename
	// for more information
	pub async fn delete_file(&self, filepath: &str) -> Result<(), Error> {
		self.check_write_password_ok()?;
		let delete_filepath: String = self.validate_filepath(filepath)?;
		let delete_file_url: String = format!(
			"{}/{}",
			self.get_files_root_url(),
			delete_filepath
		);
		let write_password = self.config.write_password.clone().unwrap();
		let delete_file_result = self.delete(
			&delete_file_url,
			&write_password
		).await;
		return delete_file_result;
	}

	// pub async fn download_file(&self, filepath: &str) -> Result<(), Error> {
	// 	let used_filepath: String = self.validate_filepath(filepath)?;

	// 	return Ok(());
	// }

	
}

#[cfg(test)]
mod files_test {
	use super::*;

	#[tokio::test]
	async fn test_get_files() {
		let client_result = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		// Directory Exists
		let test_existant_directory = "/Test";
		let existing_directory_result = client_result
			.as_ref()
			.unwrap()
			.get_files(test_existant_directory)
			.await;
		if let Err(existant_directory_error) = &existing_directory_result {
			println!("Failed Retrieving Files - Error: {}", existant_directory_error.to_string());
		}
		// println!("Files Count: {}", existing_directory_result.unwrap().len());
		// Directory does not Exists
		let test_non_existant_file_directory = "/MyNonExistantDirectory";
		let non_existant_directory_result = client_result
			.as_ref()
			.unwrap()
			.get_files(test_non_existant_file_directory)
			.await;
		assert!(non_existant_directory_result.is_ok());
	}

	#[tokio::test]
	async fn test_upload_file() {
		let client_result = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		// Without Target
		let test_image_file: &str = "./tests/files/Test_Image.jpg";
		let upload_image_no_target_result = &client_result
			.as_ref()
			.unwrap()
			.upload_file(test_image_file, None)
			.await;

		if let Err(upload_image_no_target_error) = &upload_image_no_target_result {
			println!("Failed Uploading File - Error: {} [No Target]", upload_image_no_target_error.to_string());
		}
		// With Invalid Target
		assert!(upload_image_no_target_result.is_ok());
		let test_invalid_target_filepath: &str = "/tests/files/Test_Image.jpeg";
		let upload_image_invalid_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_invalid_target_filepath)
			).await;
		assert_eq!(upload_image_invalid_target_result.is_ok(), false);
		// With Valid Target
		let test_valid_target_filepath: &str = "/tests/files/new/Test_Image.jpg";
		let upload_image_valid_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_valid_target_filepath)
			).await;
		assert!(upload_image_valid_target_result.is_ok());
		// With Root as Target
		let test_root_target_filepath: &str = "/Test_Image.jpg";
		let upload_image_root_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_root_target_filepath)
			).await;
		assert!(upload_image_root_target_result.is_ok());
		// With Valid Target and New Name to File
		let test_valid_target_filepath: &str = "/tests/files/new/New_Test_Image_Name.jpg";
		let upload_image_valid_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_valid_target_filepath)
			).await;
		assert!(upload_image_valid_target_result.is_ok());
	}

	#[tokio::test]
	async fn test_delete_file() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		// Upload the File Beforehand, so we know it exists
		let test_image_file: &str = "./tests/files/Test_Image.jpg";
		let test_delete_valid_target_filepath: &str = "/tests/files/new/path/New_Test_Image_Name.jpg";
		let test_upload_image_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file,
				Some(test_delete_valid_target_filepath)
		).await;
		assert!(test_upload_image_result.is_ok());
		let test_delete_existant_file_result = client_result
			.as_ref()
			.unwrap()
			.delete_file(test_delete_valid_target_filepath)
			.await;
		assert!(test_delete_existant_file_result.is_ok());
	}

	#[tokio::test]
	async fn test_delete_directory() {
		// Valid Directories must always be followed by a trailing / in Bunny Storage
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_image_file: &str = "./tests/files/Test_Image.jpg";
		let test_delete_valid_target_filepath: &str = "/tests/files/delete/directory/New_Test_Image_Name.jpg";
		// Upload the File for testing deleting the directory
		let test_upload_image_for_directory_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file,
				Some(test_delete_valid_target_filepath)
		).await;
		assert!(test_upload_image_for_directory_result.is_ok());
		// Test Invalid Delete the Directory ()
		let test_delete_invalid_directory_path: &str = "/tests/files/delete/directory";
		let test_delete_invalid_directory_result = client_result
			.as_ref()
			.unwrap()
			.delete_file(test_delete_invalid_directory_path)
			.await;
		assert_eq!(test_delete_invalid_directory_result.is_ok(), false);
		// Test Non Existant Delete the Directory (directories must always be followed by a trailing / in Bunny Storage)
		let test_delete_non_existing_directory_path: &str = "/directory/which/does/not/exist/";
		let test_delete_non_existing_invalid_directory_result = client_result
			.as_ref()
			.unwrap()
			.delete_file(test_delete_non_existing_directory_path)
			.await;
		assert_eq!(test_delete_non_existing_invalid_directory_result.is_ok(), false);
		// Test Valid Delete the Directory (directories must always be followed by a trailing / in Bunny Storage)
		let test_delete_valid_directory_path: &str = "/tests/files/delete/directory/";
		let test_delete_existant_file_result = client_result
			.as_ref()
			.unwrap()
			.delete_file(test_delete_valid_directory_path)
			.await;
		assert!(test_delete_existant_file_result.is_ok());
	}
}