//! **Private**; utilities for fetching configuration values from contents of files at paths defined by environment variables.

use std::ffi::OsStr;
use std::io::Read;

/// Get the contents of the file at the path specified by the requested environment variable plus `_FILE`.
pub fn get(key: &OsStr) -> Option<String> {
	let mut key: std::ffi::OsString = key.to_os_string();
	key.push("_FILE");
	let path = std::env::var(key).ok()?;

	let path = std::ffi::OsString::from(path);
	let path = std::path::PathBuf::from(path);

	let mut file = std::fs::File::open(&path)
		.unwrap_or_else(|_| panic!("to be able to open file at {path:?}"));

	let mut data = String::new();
	file.read_to_string(&mut data)
		.unwrap_or_else(|_| panic!("to be able to read from file at {path:?}"));

	Some(data)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::testing::tempfile_fixture;

	#[test]
	fn it_works() {
		let file = tempfile_fixture("XYZ");
		std::env::set_var("LETTERS_FILE", file.as_os_str());

		let value = get("LETTERS".as_ref());
		assert_eq!(value, Some("XYZ".to_string()));
	}

	#[test]
	fn missing_envvar() {
		std::env::remove_var("THIS_ENVVAR_DOES_NOT_EXIST_FILE");
		let value = get("THIS_ENVVAR_DOES_NOT_EXIST".as_ref());
		assert_eq!(value, None)
	}

	#[test]
	#[should_panic]
	fn missing_file() {
		std::env::set_var("NONEXISTENT_FILE", "/this/file/does/not/exist");
		let value = get("NONEXISTENT".as_ref());
		println!("{:?}", value);
	}
}
