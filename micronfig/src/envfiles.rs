//! Contents of files at paths defined by environment variables.

use std::io::Read;

/// Get the contents of the file at the path specified by the requested environment variable plus `_FILE`.
pub fn get<Key>(key: Key) -> Option<String>
	where Key: AsRef<std::ffi::OsStr>,
{
	let path = std::env::var(format!("{key}_FILE")).ok()?;

	let path = std::ffi::OsString::from(path);
	let path = std::path::PathBuf::from(path);

	let mut file = std::fs::File::open(&path)
		.expect(&*format!("to be able to open file at {path:?}"));

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect(&*format!("to be able to read from file at {path:?}"));

	Some(data)
}
