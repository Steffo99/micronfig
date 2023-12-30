//! Contents of files at paths defined by environment variables.

/// Get the contents of the file at the path specified by the given environment variable.
pub fn get<Key>(key: Key) -> Option<String>
	where Key: AsRef<std::ffi::OsStr>,
{
	let path = std::env::var(key).ok()?;

	let path = std::ffi::OsString::from(path);
	let path = std::path::PathBuf::from(path);

	let mut file = std::fs::File::open(&path)
		.expect(&*format!("to be able to open file at {path:?}"));

	use std::io::Read;
	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect(&*format!("to be able to read from file at {path:?}"));

	Some(data)
}
