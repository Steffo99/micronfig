//! Environment variables.

use std::ffi::OsStr;

/// Get the specified environment variable.
pub fn get(key: &OsStr) -> Option<String> {
	std::env::var(key).ok()
}
