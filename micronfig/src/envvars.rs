//! Environment variables.

use std::ffi::OsStr;

/// Get the specified environment variable.
pub fn get(key: &OsStr) -> Option<String> {
	std::env::var(key).ok()
}


#[cfg(test)]
pub(crate) mod tests {
	use super::*;

	#[test]
	fn it_works() {
		std::env::set_var("LETTERS", "XYZ");
		let value = get("LETTERS".as_ref());
		assert_eq!(value, Some("XYZ".to_string()));
	}

	#[test]
	fn missing_envvar() {
		std::env::remove_var("THIS_ENVVAR_DOES_NOT_EXIST");
		let value = get("THIS_ENVVAR_DOES_NOT_EXIST".as_ref());
		assert_eq!(value, None);
	}
}
