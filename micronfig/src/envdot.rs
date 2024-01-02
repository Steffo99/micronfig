//! Utilities for fetching configuration values defined in specific `.env` files.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

/// The type of a parsed `.env` file.
pub type DotEnv = HashMap<OsString, String>;

/// Parse a `.env` file.
pub fn parse_dotenv<P>(value: P) -> DotEnv
	where P: AsRef<Path> + Debug
{
	let mut file = File::open(&value)
		.expect(&*format!("to be able to open {value:?}"));

	let mut contents: String = String::new();
	file.read_to_string(&mut contents)
		.expect(&*format!("to be able to read {value:?}"));

	let mut keys: HashMap<OsString, String> = HashMap::new();

	let re = Regex::new(r#"^(?:export\s+)?([^=]+)\s*=\s*(.+)$"#)
		.expect("Regex to be valid");

	let _ = contents.split("\n")
		.filter_map(|line| re.captures(line))
		.map(|capture| {
			let key = &capture[0];
			let value = &capture[1];

			if value.starts_with('\'') && value.ends_with('\'') {
				(
					key.into(),
					value
						.strip_prefix('\'')
						.expect("apostrophe to be prefixed to the value")
						.strip_suffix('\'')
						.expect("apostrophe to be suffixed to the value")
						.to_owned()
				)
			}
			else if value.starts_with('"') && value.ends_with('"') {
				(
					key.into(),
					value
						.strip_prefix('"')
						.expect("quotes to be prefixed to the value")
						.strip_suffix('"')
						.expect("quotes to be suffixed to the value")
						.to_owned()
				)
			}
			else {
				(
					key.into(),
					value.to_owned()
				)
			}
		})
		.map(|(key, value)| keys.insert(key, value));

	keys
}

/// Get the requested variable from a [`DotEnv`] structure.
pub fn get(dotenv: &DotEnv, key: &OsStr) -> Option<String>
{
	dotenv.get(key).map(|v| v.to_owned())
}
