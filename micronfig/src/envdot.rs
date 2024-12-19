//! **Private**; utilities for fetching configuration values defined in specific `.env` files.

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
///
/// Returns [`None`] if no such file is found.
pub fn parse_dotenv<P>(value: P) -> Option<DotEnv>
	where P: AsRef<Path> + Debug
{
	let mut file = File::open(&value).ok()?;

	let mut contents: String = String::new();
	file.read_to_string(&mut contents)
		.unwrap_or_else(|_| panic!("to be able to read {value:?}"));

	let mut keys: HashMap<OsString, String> = HashMap::new();

	let re = Regex::new(r#"^\s*(?:export\s)?\s*([^=]+?)\s*=\s*(.+)\s*$"#)
		.expect("Regex to be valid");

	contents.split("\n")
		.filter_map(|line| re.captures(line))
		.map(|capture| {
			let key = &capture[1];
			let value = &capture[2];

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
		.for_each(|(key, value)| {
			keys.insert(key, value);
		});

	Some(keys)
}

/// Get the requested variable from a [`DotEnv`] structure.
pub fn get(dotenv: &DotEnv, key: &OsStr) -> Option<String>
{
	dotenv.get(key).map(|v| v.to_owned())
}

//noinspection DotEnvSpaceAroundSeparatorInspection
#[cfg(test)]
mod tests {
	use super::*;
	use crate::testing::tempfile_fixture;

	#[test]
	fn dotenv_simple() {
		let file = tempfile_fixture(
			// language=dotenv
			r#"
				GARAS=garas
				AUTO= auto
				BUS = bus
			"#
		);

		let parsed = parse_dotenv(file);

		let mut compared: HashMap<OsString, String> = HashMap::new();
		compared.insert("GARAS".into(), "garas".into());
		compared.insert("AUTO".into(), "auto".into());
		compared.insert("BUS".into(), "bus".into());

		assert_eq!(parsed, Some(compared));
	}

	#[test]
	fn dotenv_apos() {
		let file = tempfile_fixture(
			// language=dotenv
			r#"
				GARAS='garas'
				AUTO= 'auto'
				BUS = 'bus'
			"#
		);

		let parsed = parse_dotenv(file);

		let mut compared: HashMap<OsString, String> = HashMap::new();
		compared.insert("GARAS".into(), "garas".into());
		compared.insert("AUTO".into(), "auto".into());
		compared.insert("BUS".into(), "bus".into());

		assert_eq!(parsed, Some(compared));
	}

	#[test]
	fn dotenv_quote() {
		let file = tempfile_fixture(
			// language=dotenv
			r#"
				GARAS="garas"
				AUTO= "auto"
				BUS = "bus"
			"#
		);

		let parsed = parse_dotenv(file);

		let mut compared: HashMap<OsString, String> = HashMap::new();
		compared.insert("GARAS".into(), "garas".into());
		compared.insert("AUTO".into(), "auto".into());
		compared.insert("BUS".into(), "bus".into());

		assert_eq!(parsed, Some(compared));
	}

	#[test]
	fn dotenv_export() {
		let file = tempfile_fixture(
			// language=dotenv
			r#"
				export GARAS=garas
				export AUTO= auto
				export BUS = bus
			"#
		);

		let parsed = parse_dotenv(file);

		let mut compared: HashMap<OsString, String> = HashMap::new();
		compared.insert("GARAS".into(), "garas".into());
		compared.insert("AUTO".into(), "auto".into());
		compared.insert("BUS".into(), "bus".into());

		assert_eq!(parsed, Some(compared));
	}
}