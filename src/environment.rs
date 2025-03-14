use std::i32;

use crate::errors::Error;

pub fn get_string_from_env(env_key: &str) -> Result<String, Error> {
	return std::env::var(env_key)
		.map_err(|env_key_error| Error::new_from_message(&format!(
			"Failed retrieving environment variable. Check {} in .env - Error {}",
			env_key,
			env_key_error.to_string())
		));
}

pub fn get_non_empty_string_from_env(env_key: &str) -> Result<String, Error> {
	let env_content = get_string_from_env(env_key)?;
	let trimmed_env_content = env_content.trim();
	if trimmed_env_content.is_empty() {
		return Err(Error::new_from_message(&format!(
			"Invalid environment content. Must not be empty. Check {}",
			env_key
		)));
	}
	return Ok(trimmed_env_content.to_string());
}

pub fn get_i8_from_env(env_key: &str) -> Result<i8, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_i8_result = env_content.parse::<i8>();
	match env_i8_result {
		Ok(env_i8) => Ok(env_i8),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_i16_from_env(env_key: &str) -> Result<i16, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_i16_result = env_content.parse::<i16>();
	match env_i16_result {
		Ok(env_i16) => Ok(env_i16),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_i32_from_env(env_key: &str) -> Result<i32, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_i32_result = env_content.parse::<i32>();
	match env_i32_result {
		Ok(env_i32) => Ok(env_i32),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_i64_from_env(env_key: &str) -> Result<i64, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_i64_result = env_content.parse::<i64>();
	match env_i64_result {
		Ok(env_i64) => Ok(env_i64),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_u8_from_env(env_key: &str) -> Result<u8, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_u8_result = env_content.parse::<u8>();
	match env_u8_result {
		Ok(env_u8) => Ok(env_u8),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_u16_from_env(env_key: &str) -> Result<u16, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_u16_result = env_content.parse::<u16>();
	match env_u16_result {
		Ok(env_u16) => Ok(env_u16),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_u32_from_env(env_key: &str) -> Result<u32, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_u32_result = env_content.parse::<u32>();
	match env_u32_result {
		Ok(env_u32) => Ok(env_u32),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}

pub fn get_u64_from_env(env_key: &str) -> Result<u64, Error> {
	let env_content = get_non_empty_string_from_env(env_key)?;
	let env_u64_result = env_content.parse::<u64>();
	match env_u64_result {
		Ok(env_u64) => Ok(env_u64),
		Err(parse_error) => Err(Error::new_from_message(&parse_error.to_string())),
	}
}