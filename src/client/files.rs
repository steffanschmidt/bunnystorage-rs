use std::{collections::HashMap, ffi::OsStr, fs, io::{Read, Write}, path::{self, PathBuf}};
use futures::StreamExt;

use serde_json::Value;

use crate::{errors::Error, models::file::File};

use super::{BunnyCDNClient, BunnyCDNDataOptions, ContentType, ACCESS_KEY_HEADER_NAME, CONTENT_TYPE_HEADER_NAME};


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

	/// This function add a trailing / to the directory provided in case there is
	/// one missing. This is done to avoid having to do it manually every single time
	/// Unlike for delete directory then this has no consequences, therefore this will
	/// not error out if the trailing / is missing.
	/// Parameters:
	/// * 	directory -> relative to the root directory of the storage name

	/// See https://docs.bunny.net/reference/get_-storagezonename-path- for documentation
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
		let files_value_result: Value = serde_json::from_str(&files_response.body)
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

	fn attempt_make_filepath_absolute(&self, filepath: &str) ->  Result<PathBuf, Error> {
		let trimmed_filepath = filepath.trim();
		let absolute_filepath_buffer = path::absolute(trimmed_filepath)
			.map_err(|absolute_filepath_buffer_error| Error::new_from_message(&absolute_filepath_buffer_error.to_string()))?;
		
		return Ok(absolute_filepath_buffer);
	}

	fn attempt_get_absolute_filepath(&self, filepath: &str) -> Result<String, Error> {
		let trimmed_filepath = filepath.trim();
		let absolute_filepath_buffer = self.attempt_make_filepath_absolute(trimmed_filepath)?;
		let filepath_option = absolute_filepath_buffer.to_str();
		return match filepath_option {
			None => Err(Error::new_from_message("Failed to retrieve the absolute filepath")),
			Some(filepath) => Ok(filepath.to_string()),
		}
	}

	fn validate_local_filepath(&self, filepath: &str, require_file_exist: bool) -> Result<String, Error> {
		let validated_filepath = self.validate_filepath(filepath)?;
		let absolute_filepath_buffer = self.attempt_make_filepath_absolute(&validated_filepath)?;
		if require_file_exist {
			let file_exists = fs::exists(&absolute_filepath_buffer)
				.map_err(|file_exists_error| Error::new_from_message(&file_exists_error.to_string()))?;
	
			if !file_exists {
				return Err(Error::new_from_message("Invalid Filepath. File does not exist"));
			}
		}
		return self.attempt_get_absolute_filepath(&validated_filepath);
	}

	fn retrieve_filepath_extension(&self, filepath: &str) -> String {
		let trimmed_filepath = filepath.trim();
		let filepath_extension_opt = path::Path::new(trimmed_filepath)
			.extension()
			.and_then(OsStr::to_str);
		
		match filepath_extension_opt {
			Some(filepath_extension) => filepath_extension.to_string(),
			None => "".to_string(),
		}
	}

	fn retrieve_filepath_filename(&self, filepath: &str) -> String {
		let trimmed_filepath = filepath.trim();
		let filepath_filename_opt = path::Path::new(trimmed_filepath)
			.file_name()
			.and_then(OsStr::to_str);

		match filepath_filename_opt {
			Some(filepath_filename) => filepath_filename.to_string(),
			None => "".to_string(),
		}
	}

	fn prepare_remote_filepath(&self, filepath: &str) -> Result<String, Error> {
		let mut validated_filepath: String = self.validate_filepath(filepath)?;
		if validated_filepath.starts_with(".") {
			validated_filepath.remove(0);
		}
		if validated_filepath.starts_with("/") {
			validated_filepath.remove(0);
		}
		return Ok(validated_filepath);
	}

	fn derive_filepath_common(&self, source_filepath: &str, target_filepath: &str) -> Result<String, Error> {
		let validated_source_filepath = self.validate_filepath(source_filepath)?;
		let trimmed_target_filepath = target_filepath.trim();
		if trimmed_target_filepath.is_empty() {
			return Ok(validated_source_filepath);
		}
		// Check if there is an extension present on both and if they are equal
		let local_file_extension: String = self.retrieve_filepath_extension(&validated_source_filepath);
		let remote_file_extension: String = self.retrieve_filepath_extension(trimmed_target_filepath);
		if !local_file_extension.is_empty() && !remote_file_extension.is_empty() && local_file_extension != remote_file_extension {
			return Err(Error::new_from_message(&format!("Invalid Remote File Extension - Expected {}, Received {}", local_file_extension, remote_file_extension)));
		}
		// If there is a local file extension but no file extension on the remote filepath,
		// then it is assumed to be a directory and the filename from the original file
		// will be appended to the directory
		let mut derived_remote_path: String = target_filepath.to_owned();
		let local_filename: String = self.retrieve_filepath_filename(&validated_source_filepath);
		if !local_filename.is_empty() && remote_file_extension.is_empty() {
			derived_remote_path = format!("{}/{}", target_filepath, local_filename);
		}
		return Ok(derived_remote_path);
	}

	fn derive_remote_filepath(&self, local_filepath: &str, remote_filepath: &str) -> Result<String, Error> {
		let derived_remote_path: String = self.derive_filepath_common(local_filepath, remote_filepath)?;
		return self.prepare_remote_filepath(&derived_remote_path);
	}

	fn evaluate_remote_target_filepath(&self, local_filepath: &str, remote_filepath: Option<&str>) -> Result<String, Error> {
		return match remote_filepath {
			None => self.prepare_remote_filepath(local_filepath),
			Some(provided_remote_filepath) => self.derive_remote_filepath(local_filepath, provided_remote_filepath),
		};
	}

	///	If the target is not provided, then the source filepath will be used.
	///	For example say that a file is stored on /my/test/directory/file.json, then
	///	the target, if not provided, will be uploaded to that in Bunny Storage.
	///	However is a target is provided, then this will be used instead.
	///	The target must be filepath to allow for renaming a file and the
	///	file extensions must match.
	/// 
	/// Parameters:
	/// 	local_filepath: absolute filepath to a local file
	/// 	remote_filepath: If provided, absolute filepath to a path on Bunnystorage

	/// If the remote_filepath is not provided, then the local_filepath is used.
	/// See https://docs.bunny.net/reference/put_-storagezonename-path-filename for documentation
	pub async fn upload_file(&self, local_filepath: &str, remote_filepath: Option<&str>) -> Result<(), Error> {
		self.check_write_password_ok()?;
		// Evaluate both target and source filepath
		let used_local_filepath: String = self.validate_local_filepath(local_filepath, true)?;
		let used_remote_filepath: String = self.evaluate_remote_target_filepath(local_filepath, remote_filepath)?;
		// Read the file contents in a vector
		let mut file_contents = Vec::new();
		let mut file = fs::File::open(used_local_filepath)
			.map_err(|open_file_error| Error::new_from_message(&open_file_error.to_string()))?;
		
		let read_file_result = file.read_to_end(&mut file_contents);
		if let Err(read_file_error) = read_file_result {
			return Err(Error::new_from_message(&format!("Failed Read File - {}", read_file_error.to_string())));
		}
		let upload_file_url: String = format!(
			"{}/{}",
			self.get_files_root_url(),
			used_remote_filepath,
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

	/*
		This is an abstraction for deleting files and directories
		The reason for this is that directories require a trailing /, which is not
		Parameters:
			entry_path: Either a directory or a filepath
	 */
	async fn handle_delete_entry(&self, entry_path: &str) -> Result<(), Error> {
		self.check_write_password_ok()?;
		let delete_entry_path: String = self.validate_filepath(entry_path)?;
		let delete_file_url: String = format!(
			"{}/{}",
			self.get_files_root_url(),
			delete_entry_path
		);
		let write_password = self.config.write_password.clone().unwrap();
		let delete_file_result = self.delete(
			&delete_file_url,
			&write_password
		).await;
		return delete_file_result;
	}

	/*
		There does not seem to be any constraint on what files may or may not be provided
		except it may not end with a trailing /. This errors if this is the case, since otherwise
		some unexpected outcomes by occur e.g. one accidently deleting an entire directory rather
		than a single file.
		Parameters:
			filepath: relative to the root
	*/
	pub async fn delete_file(&self, filepath: &str) -> Result<(), Error> {
		if filepath.ends_with("/") {
			return Err(Error::new_from_message(&format!("Invalid Filepath - Provided: {}. Has a trailing /. Trying to delete a directory?", filepath)));
		}
		return self.handle_delete_entry(filepath).await;
	}

	/*
		A directory path must be followed by a trailing /. Otherwise it will not be found even if it exists
		For example if /my/test/directory exists on Bunnystorage and we want to delete it, then
		we must supply the path /my/test/directory/
	 */
	pub async fn delete_directory(&self, directory_path: &str) -> Result<(), Error> {
		if !directory_path.ends_with("/") {
			return Err(Error::new_from_message(&format!("Invalid Directory Path. Missing trailing / - Provided: {}", directory_path)));
		}
		return self.handle_delete_entry(directory_path).await;
	}

	fn derive_local_filepath(&self, remote_filepath: &str, local_filepath: &str) -> Result<String, Error> {
		let derived_local_filepath = self.derive_filepath_common(remote_filepath, local_filepath)?;
		return self.attempt_get_absolute_filepath(&derived_local_filepath);
	}

	fn evaluate_local_target_filepath(&self, remote_filepath: &str, local_filepath: Option<&str>) -> Result<String, Error> {
		match local_filepath {
			None => self.attempt_get_absolute_filepath(remote_filepath),
			Some(provided_local_filepath) => {
				return self.derive_local_filepath(remote_filepath, provided_local_filepath);
			}
		}
	}

	/*
		This function handles streaming the contents of the file into either a vector or a file pointer.
		In case of the latter is present, then the content will be consumed by the file and not be written
		to the vector. If the file is not present the content will be added to a vector.
		The vector is returned in both cases, but if a file is present, then it will be empty.
		Parameters:
			remote_filepath: The filepath on bunnystorage relative to the root
			file: file opened in another function, allows for streaming content into the file
	*/
	async fn handle_get_and_stream_file_contents(&self, remote_filepath: &str, mut file: Option<&mut fs::File>) -> Result<Vec<u8>, Error> {
		let used_remote_filepath: String = self.prepare_remote_filepath(remote_filepath)?;
		let download_file_url = format!(
			"{}/{}",
			self.get_files_root_url(),
			used_remote_filepath,
		);
		let download_file_request = self.http_client.get(&download_file_url)
			.header(ACCESS_KEY_HEADER_NAME, &self.config.read_password);

		let http_download_file_response = download_file_request.send()
			.await
			.map_err(|http_downlad_file_error| Error::new_from_message(&http_downlad_file_error.to_string()))?
			.error_for_status()
			.map_err(|http_get_file_error| Error::new_from_message(&http_get_file_error.to_string()))?;

		// Setup 
		let mut file_contents: Vec<u8> = Vec::new();
		let mut file_stream = http_download_file_response.bytes_stream();
		while let Some(file_item_result) = file_stream.next().await {
			if let Err(file_item_error) = file_item_result {
				return Err(Error::new_from_message(&format!("Failed Stream Contents - Error: {}", file_item_error.to_string())));
			}
			let file_item_content = &file_item_result.unwrap().to_vec();
			if let Some(ref mut file_pointer) = file {
				let write_file_result = file_pointer.write(file_item_content);
				if let Err(write_file_error) = write_file_result {
					return Err(Error::new_from_message(&format!("Failed Write File Contents - Error: {}", write_file_error.to_string())));
				}
			} else {
				file_contents.extend(file_item_content);
			}
		}
		return Ok(file_contents);
	}

	/// This function downloads the contents from Bunnystorage into a local file.
	/// If it is not possible to open a file to the provided location, then an error
	/// will be thrown. 
	/// 
	/// Otherwise the contents will be streamed into the file to limit the memory footprint. 
	/// In case the streaming fails the file will be deleted.
	///	
	/// Parameters:
	///	*	remote_filepath: The filepath on bunnystorage
	///	*	local_filepath: Local Directory or filepath
	/// 
	/// Note: This is handled by the internal function 'handle_get_and_stream_file_contents'
	/// 
	/// # Examples
	/// ```
	/// ```
	/// let downfile_file_result: Result<(), Error> = download_file(my_remote_filepath, my_local_filepath);
	pub async fn download_file(&self, remote_filepath: &str, local_filepath: &str) -> Result<(), Error> {
		let used_local_filepath: String = self.evaluate_local_target_filepath(remote_filepath, Some(local_filepath))?;
		let mut local_file = fs::File::create(&used_local_filepath)
			.map_err(|open_file_error| Error::new_from_message(&open_file_error.to_string()))?;

		let download_file_content_result = self.handle_get_and_stream_file_contents(remote_filepath, Some(&mut local_file)).await;
		if let Err(download_file_content_error) = download_file_content_result {
			_ = fs::remove_file(used_local_filepath);
			return Err(download_file_content_error);
		}
		return Ok(());
	}
	
	/// This function retrieves the contents from Bunnystorage and add them to a vector
	/// 
	///	Parameters:
	///	*	remote_filepath: The filepath on bunnystorage
	/// Note: This is handled by the internal function 'handle_get_and_stream_file_contents'
	pub async fn download_file_content(&self, remote_filepath: &str) -> Result<Vec<u8>, Error> {
		return self.handle_get_and_stream_file_contents(
			remote_filepath,
			None
		).await;
	}

}

#[cfg(test)]
mod files_tests {
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
		let test_image_file: &str = "./tests/files/source/Test_Image.jpg";
		let upload_image_no_target_result = &client_result
			.as_ref()
			.unwrap()
			.upload_file(test_image_file, None)
			.await;
		assert!(upload_image_no_target_result.is_ok());
		if let Err(upload_image_no_target_error) = &upload_image_no_target_result {
			println!("Failed Uploading File - Error: {} [No Target]", upload_image_no_target_error.to_string());
		}
		// Without Target and Extension
		let test_no_extension_file: &str = "./tests/files/source/Test_NoExtension";
		let upload_image_no_extension_and_no_target_result = &client_result
			.as_ref()
			.unwrap()
			.upload_file(test_no_extension_file, None)
			.await;
		assert!(upload_image_no_extension_and_no_target_result.is_ok());
		// With Invalid Target
		let test_invalid_remote_filepath: &str = "/tests/files/source/Test_Image.jpeg";
		let upload_image_invalid_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_invalid_remote_filepath)
			).await;
		assert_eq!(upload_image_invalid_target_result.is_ok(), false);
		// With Valid Target
		let test_valid_remote_filepath: &str = "/tests/files/source/new/Test_Image.jpg";
		let upload_image_valid_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_valid_remote_filepath)
			).await;
		assert!(upload_image_valid_target_result.is_ok());
		// With Root as Target
		let test_root_remote_filepath: &str = "/Test_Image.jpg";
		let upload_image_root_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_root_remote_filepath)
			).await;
		assert!(upload_image_root_target_result.is_ok());
		// With Directory as Target
		let test_directory_remote_filepath: &str = "/NewFolder";
		let upload_image_root_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file,
				Some(test_directory_remote_filepath)
			).await;
		assert!(upload_image_root_target_result.is_ok());
		// With Valid Target and New Name to File
		let test_valid_remote_filepath: &str = "/tests/files/source/new/New_Test_Image_Name.jpg";
		let upload_image_valid_target_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file, 
				Some(test_valid_remote_filepath)
			).await;
		assert!(upload_image_valid_target_result.is_ok());
	}

	#[tokio::test]
	async fn test_delete_file() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		// Upload the File Beforehand, so we know it exists
		let test_image_file: &str = "./tests/files/source/Test_Image.jpg";
		let test_delete_valid_remote_filepath: &str = "/tests/files/source/new/path/New_Test_Image_Name.jpg";
		let test_upload_image_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file,
				Some(test_delete_valid_remote_filepath)
		).await;
		assert!(test_upload_image_result.is_ok());
		let test_delete_existant_file_result = client_result
			.as_ref()
			.unwrap()
			.delete_file(test_delete_valid_remote_filepath)
			.await;
		assert!(test_delete_existant_file_result.is_ok());
	}

	#[tokio::test]
	async fn test_delete_directory() {
		// Valid Directories must always be followed by a trailing / in Bunny Storage
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_image_file: &str = "./tests/files/source/Test_Image.jpg";
		let test_delete_valid_remote_filepath: &str = "/tests/files/source/delete/directory/New_Test_Image_Name.jpg";
		// Upload the File for testing deleting the directory
		let test_upload_image_for_directory_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file,
				Some(test_delete_valid_remote_filepath)
		).await;
		assert!(test_upload_image_for_directory_result.is_ok());
		// Test Invalid Delete the Directory
		let test_delete_invalid_directory_path: &str = "/tests/files/source/delete/directory";
		let test_delete_invalid_directory_result = client_result
			.as_ref()
			.unwrap()
			.delete_directory(test_delete_invalid_directory_path)
			.await;
		assert_eq!(test_delete_invalid_directory_result.is_ok(), false);
		// Test Non Existant Delete the Directory (directories must always be followed by a trailing / in Bunny Storage)
		let test_delete_non_existing_directory_path: &str = "/directory/which/does/not/exist";
		let test_delete_non_existing_invalid_directory_result = client_result
			.as_ref()
			.unwrap()
			.delete_directory(test_delete_non_existing_directory_path)
			.await;
		assert_eq!(test_delete_non_existing_invalid_directory_result.is_ok(), false);
		// Test Valid Delete the Directory
		let test_delete_valid_directory_path: &str = "/tests/files/source/delete/directory/";
		let test_delete_valid_directory_result = client_result
			.as_ref()
			.unwrap()
			.delete_directory(test_delete_valid_directory_path)
			.await;
		assert_eq!(test_delete_valid_directory_result.is_ok(), true);
	}

	#[tokio::test]
	async fn test_download_file() {
		let client_result: Result<BunnyCDNClient, Error> = BunnyCDNClient::new_from_env();
		assert!(client_result.is_ok());
		let test_image_file: &str = "./tests/files/source/Test_Image.jpg";
		// Upload the File so that we know it exists
		let test_upload_image_for_directory_result = client_result
			.as_ref()
			.unwrap()
			.upload_file(
				test_image_file,
				None,
		).await;
		assert!(test_upload_image_for_directory_result.is_ok());
		// Test Download a Existant File to File
		let test_image_target_file: &str = "./tests/files/target/Test_Image_New_File.jpg";
		let test_download_existant_file_result = client_result
			.as_ref()
			.unwrap()
			.download_file(
				test_image_file,
				test_image_target_file,
		).await;
		if let Err(test_download_existant_file_error) = &test_download_existant_file_result {
			println!("Failed Downloading Existant File - Error: {}", test_download_existant_file_error.to_string());
		}
		// Test Download a Existant File to Directory
		let test_image_target_directory: &str = "./tests/files/target";
		let test_download_existant_file_to_directory_result = client_result
			.as_ref()
			.unwrap()
			.download_file(
				test_image_file,
				test_image_target_directory,
		).await;
		assert!(test_download_existant_file_to_directory_result.is_ok());
		// Test Download a Non-Existant File
		let test_non_existant_image_file: &str = "./tests/files/source/No_Such_Files.json";
		let test_non_existant_image_target_file: &str = "./tests/files/target/No_Such_Files.json";
		let test_download_non_existant_file_result = client_result
			.as_ref()
			.unwrap()
			.download_file(
				test_non_existant_image_file,
				test_non_existant_image_target_file,
		).await;
		assert_eq!(test_download_non_existant_file_result.is_ok(), false);
	}

}