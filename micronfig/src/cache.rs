//! **Private**; definition of [`Cache`].

use std::ffi::OsStr;
use std::fmt::Debug;

/// Cache initialized only once per config block and used to quickly retrieve configuration values.
///
/// Every `env*` feature has its own field here, which may or may not be used.
#[derive(Clone, Default, Debug)]
pub struct Cache {
	/// Unused.
	#[cfg(feature = "envvars")]
	pub envvars: (),

	/// Unused.
	#[cfg(feature = "envfiles")]
	pub envfiles: (),

	/// `.env` file cache, in order of access priority.
	///
	/// More can be added with [`Cache::envdot_register`].
	#[cfg(feature = "envdot")]
	pub envdot: Vec<crate::envdot::DotEnv>
}

impl Cache {
	/// Initialize a new cache.
	pub fn new() -> Self {
		let mut this = Self::default();

		if cfg!(feature = "envdot") {
			this.envdot_register("./.env.local");
			this.envdot_register("./.env");
		}

		this
	}

	/// Get a value from the cache.
	///
	/// The following sources, if the respective feature is enabled, are checked in the following order:
	/// 1. `envfiles`
	/// 2. `envvars`
	/// 3. `envdot`
	///
	pub fn get(&self, key: &OsStr) -> Option<String>
	{
		let mut value = None;

		if cfg!(feature = "envfiles") {
			value = crate::envfiles::get(key);
		}

		if cfg!(feature = "envvars") && value.is_none() {
			value = crate::envvars::get(key);
		}

		if cfg!(feature = "envdot") && value.is_none() {
			for dotenv in self.envdot.iter() {
				value = crate::envdot::get(dotenv, key);
				if value.is_some() {
					break;
				}
			}
		}

		value
	}

	/// Register a new `.env` file in the cache, if it exists.
	#[cfg(feature = "envdot")]
	pub fn envdot_register<Path>(&mut self, path: Path)
		where Path: AsRef<std::path::Path> + Debug
	{
		let dotenv = crate::envdot::parse_dotenv(path);
		if let Some(dotenv) = dotenv {
			self.envdot.push(dotenv);
		}
	}
}

//noinspection DotEnvSpaceAroundSeparatorInspection
#[cfg(test)]
mod tests {
	use crate::testing::tempfile_fixture;
	use super::*;

	#[cfg(feature = "envdot")]
	#[test]
	fn envdot_register() {
		let file = tempfile_fixture(
			// language=dotenv
			r#"
				GARAS=garas
				export AUTO= auto
				BUS = bus
			"#
		);

		let mut cache = Cache::default();
		cache.envdot_register(file.as_os_str());

		assert_eq!(cache.envdot.len(), 1);
	}

	#[cfg(feature = "envvars")]
	#[test]
	fn get_envvars() {
		std::env::set_var("GARAS", "garas");
		std::env::remove_var("GARAS_FILE");

		let cache = Cache::default();
		assert_eq!(cache.get("GARAS".as_ref()), Some("garas".to_string()));
	}

	#[cfg(feature = "envfiles")]
	#[test]
	fn get_envfiles() {
		let file = tempfile_fixture("garas");
		std::env::remove_var("GARAS");
		std::env::set_var("GARAS_FILE", file.as_os_str());

		let cache = Cache::default();
		assert_eq!(cache.get("GARAS".as_ref()), Some("garas".to_string()));
	}

	#[cfg(feature = "envdot")]
	#[test]
	fn get_envdot() {
		std::env::remove_var("GARAS");
		std::env::remove_var("GARAS_FILE");
		let file = tempfile_fixture(
			// language=dotenv
			r#"GARAS=garas"#
		);

		let mut cache = Cache::default();
		cache.envdot_register(file.as_os_str());
		assert_eq!(cache.get("GARAS".as_ref()), Some("garas".to_string()));
	}

	#[test]
	fn priority() {
		let mut cache = Cache::default();

		let envfiles_file = tempfile_fixture("envfiles");

		let envdot_file = tempfile_fixture(
			// language=dotenv
			r#"
				export ENVFILES=envdot
				export ENVVARS=envdot
				export ENVDOT=envdot
			"#
		);

		if cfg!(feature = "envfiles") {
			std::env::set_var("ENVFILES_FILE", envfiles_file.as_os_str());
			std::env::remove_var("ENVVARS_FILE");
			std::env::remove_var("ENVDOT_FILE");
			std::env::remove_var("NONE_FILE");
		}

		if cfg!(feature = "envvars") {
			std::env::set_var("ENVFILES", "envvars");
			std::env::set_var("ENVVARS", "envvars");
			std::env::remove_var("ENVDOT");
			std::env::remove_var("NONE");
		}

		if cfg!(feature = "envdot") {
			cache.envdot_register(envdot_file.as_os_str());
		}

		if cfg!(feature = "envfiles") {
			assert_eq!(cache.get("ENVFILES".as_ref()), Some("envfiles".to_string()));
		}

		if cfg!(feature = "envvars") {
			assert_eq!(cache.get("ENVVARS".as_ref()), Some("envvars".to_string()));
		}

		if cfg!(feature = "envdot") {
			assert_eq!(cache.get("ENVDOT".as_ref()), Some("envdot".to_string()));
		}

		assert_eq!(cache.get("NONE".as_ref()), None);
	}
}
