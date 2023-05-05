//! Lowest-level API — Manually select configuration sources.
//!
//! It can be useful if you want to specify manually the sources to access when retrieving configuration values.
//!
//! Each possible source has an associated module, and a feature named `single_{MODULENAME}` enabling it; see the list of modules below to see what sources are available!


#[cfg(feature = "single_envvars")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "single_envvars")))]
pub mod envvars;

#[cfg(feature = "single_envfiles")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "single_envfiles")))]
pub mod envfiles;

