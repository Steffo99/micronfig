//! Environment variables.

/// Get the specified environment variable.
pub fn get<Key>(key: Key) -> Option<String>
	where Key: AsRef<std::ffi::OsStr>,
{
	std::env::var(key).ok()
}
