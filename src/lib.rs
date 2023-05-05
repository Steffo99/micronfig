//! Tiny crate for simple configuration management.
//!
//! > **Unstable**; I haven't fully committed to the API yet, so it might change wildly in the following minor versions (`0.x.0`).
//!
//! # Features
//!
//! This crate handles:
//!
//! - Retrieval of values of configuration properties from multiple sources, such as environment variables or files
//! - Parsing of retrieved data
//! - Displaying human-readable errors if case a step does not succeed
//!
//! # Usage
//!
//! This crate has four levels of abstraction, each one with a different usage method.
//!
//! In order from the highest to the lowest, they are:
//!
//! 1. **Recommended**: [`required`] and [`optional`], macros which allow you to define global, lazily-evaluated, configuration values;
//! 2. [`handle::get_required`] and [`handle::get_optional`], functions which allow you to get a configuration value in a specific moment, without having to consider handling errors;
//! 3. [`multi::get`], function which behaves in the same way as the previous two, but returns [`multi::Source`] instead, allowing you to handle errors how you prefer;
//! 4. [`single`], module containing submodules allowing the retrieval of configuration values from a single source, returning a source-specific [`Result`].
//!
//! ## Examples
//!
//! Some examples are provided in the crate source, [inside the `examples/` directory](https://github.com/Steffo99/micronfig/tree/main/examples).

#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/micronfig/main/icon.png")]


pub mod single;

#[cfg(feature = "multi")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "multi")))]
pub mod multi;

#[cfg(feature = "handle")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "handle")))]
pub mod handle;

#[cfg(feature = "macros")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros")))]
pub use lazy_static;
#[cfg(feature = "macros")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "macros")))]
pub mod macros;

#[cfg(feature = "testing")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "testing")))]
pub mod testing;