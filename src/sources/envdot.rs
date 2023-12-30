//! Variables defined in specific dotenv files.

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct DotEnv(
	HashMap<String, String>
);

impl DotEnv {
	pub fn var(&self, key: &str) -> Option<&String> {
		self.0.get(key)
	}
}

impl<P> From<P> for DotEnv
	where P: AsRef<Path> + Debug
{
	fn from(value: P) -> Self {
		let mut file = File::open(&value)
			.expect(&*format!("to be able to open {value:?}"));

		let mut contents: String = String::new();
		file.read_to_string(&mut contents)
			.expect(&*format!("to be able to read {value:?}"));

		let mut keys: HashMap<String, String> = HashMap::new();

		let re = Regex::new(r#"^(?:export\s+)?([^=]+)\s*=\s*(.+)$"#)
			.expect("Regex to be valid");

		let _ = contents.split("\n")
			.filter_map(|line| re.captures(line))
			.map(|capture| (capture[0].to_owned(), capture[1].to_owned()))
			.map(|(key, value)| keys.insert(key, value));

		Self(keys)
	}
}

/// Get the contents of the file at the path specified by the given environment variable.
pub fn get(dotenv: &DotEnv, key: &str) -> Option<String>
{
	dotenv.var(key).map(|v| v.to_owned())
}
