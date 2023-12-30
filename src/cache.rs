use std::fmt::Debug;
use std::path::Path;


#[derive(Clone, Default, Debug)]
pub struct MicronfigCache {
	#[cfg(feature = "envdot")]
	pub dotenvs: Vec<crate::sources::envdot::DotEnv>
}

impl MicronfigCache {
	#[cfg(feature = "envdot")]
	pub fn add_envdot<P>(&mut self, path: P)
		where P: AsRef<Path> + Debug
	{
		self.dotenvs.push(
			crate::sources::envdot::DotEnv::from(
				path
			)
		);
	}
}