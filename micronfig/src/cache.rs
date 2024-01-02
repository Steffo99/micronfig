//! Definition of [`Cache`].

use std::fmt::Debug;

/// Cache initialized only once per config block and used to quickly retrieve configuration values.
#[derive(Clone, Default, Debug)]
pub struct Cache {
	/// `.env` file cache, in order of access priority.
	///
	/// More can be added with [`Cache::register_dotenv`].
	#[cfg(feature = "envdot")]
	pub envdot: Vec<crate::envdot::DotEnv>
}

impl Cache {
	/// Initialize a new cache.
	pub fn new() -> Self {
		let mut this = Self::default();

		if cfg!(feature = "envdot") {
			this.register_dotenv("./.env.local");
			this.register_dotenv("./.env");
		}

		this
	}

	/// Get a value from the cache.
	pub fn get<Key>(&self, key: Key) -> Option<String>
		where Key: AsRef<&std::ffi::OsStr>
	{
		let mut value = None;

		if cfg!(feature = "envfiles") {
			value = crate::envfiles::get(format!("{key}_FILE"));
		}

		if cfg!(feature = "envvars") && value.is_none() {
			value = crate::envvars::get(&key);
		}

		if cfg!(feature = "envdot") && value.is_none() {
			for dotenv in self.envdot.iter() {
				value = crate::envdot::get(dotenv, &key);
				if value.is_some() {
					break;
				}
			}
		}

		value
	}

	/// Register a new `.env` file in the cache.
	///
	/// Equivalent to adding an item to [`Cache::envdot`].
	#[cfg(feature = "envdot")]
	pub fn register_dotenv<Path>(&mut self, path: Path)
		where Path: AsRef<std::path::Path> + Debug
	{
		self.envdot.push(
			crate::envdot::DotEnv::from(path)
		);
	}
}
