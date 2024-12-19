//! Crate for macro-based configuration management.
//!
//! ## Description
//!
//! This crate and it's sister [`micronfig_macros`] combine to provide the [`config`] macro, which allows the developer to define all configuration variables required by an application in a single place and have them expanded to static references of the desired types.
//!
//! ```
//! micronfig::config! {
//! 	DATABASE_URI,
//! 	APPLICATION_NAME: String,
//! 	MAX_CONCURRENT_USERS: String > u64,
//! 	SHOWN_ALERT?,
//! }
//! ```
//!
//! ## Examples
//!
//! ### Strings configuration
//!
//! To define configuration variables returning a string, create a [`config`] block and list their names separated by commas `,`:
//!
//! ```
//! micronfig::config! {
//! 	VARIABLE_A,
//! 	VARIABLE_B,
//! 	VARIABLE_C,
//! 	PATH,
//! }
//! ```
//!
//! To access them, call their name as if it was a function:
//!
//! ```
//! # micronfig::config! {
//! # 	VARIABLE_A,
//! # 	VARIABLE_B,
//! # 	VARIABLE_C,
//! # 	PATH,
//! # }
//! #
//! # std::env::set_var("VARIABLE_A", "a");
//! # std::env::set_var("VARIABLE_B", "b");
//! # std::env::set_var("VARIABLE_C", "c");
//! # std::env::set_var("PATH", "/bin");
//! #
//! # if cfg!(feature = "envvars") {
//! // These will all return `&'static str` values.
//! println!("{}", VARIABLE_A());
//! println!("{}", VARIABLE_B());
//! println!("{}", VARIABLE_C());
//! println!("{}", PATH());
//! # }
//! ```
//!
//! > ***Note***
//! >
//! > Both the [`config`] block and variables defined in it are lazily initialized on first call.
//! >
//! > The first time one of these functions is called, configuration files will be parsed, and the first time each is called, its value is retrieved and stored.
//!
//! ### Required and optional variables
//!
//! By default, configuration variables are all required, causing a [panic] if their value is missing the first time their function is called.
//!
//! Configuration variables can be marked as [Option]al by suffixing a question mark `?` to their name, making them return a `&'static` [`Option`] instead:
//!
//! ```
//! micronfig::config! {
//! 	VARIABLE_REQUIRED,
//! 	VARIABLE_OPTIONAL?,
//! }
//! ```
//!
//! ### Conversions
//!
//! All variables are read from their source as strings; therefore, the following explicit syntax for defining them is supported:
//!
//! ```
//! micronfig::config! {
//! 	VARIABLE_A: String,
//! 	VARIABLE_B: String,
//! 	VARIABLE_C: String,
//! }
//! ```
//!
//! Strings are not the best option for most situations, so the crate makes use of some traits to allow their conversion to different types:
//!
//! | Trait | Symbol | Notes |
//! |---|---|---|
//! | [`From`] | `->` |  |
//! | [`TryFrom`] | `=>` | Will panic if the conversion fails. |
//! | [`std::str::FromStr`] | `>` | Will panic if the parsing fails. |
//!
//! The syntax for conversion is as follows:
//!
//! ```
//! use std::net::SocketAddr;
//!
//! micronfig::config! {
//!  	// use FromStr to parse the String as an isize
//! 	REQUIRED_SIGNED: String > isize,
//!  	// use FromStr to parse the String as a SocketAddr
//!  	REQUIRED_SOCKETADDR: String > SocketAddr,
//! 	// use From to convert the String to... another String
//! 	REQUIRED_STRING: String -> String,
//! 	// use TryFrom to convert the String to another String
//!  	// (there aren't many types in std to make valid examples from!)
//! 	REQUIRED_TRYSTRING: String => String,
//! 	// the conversion will not be performed for missing optional variables
//!		OPTIONAL_UNSIGNED?: String > usize,
//! }
//! ```
//!
//! Custom types can be used as well:
//!
//! ```
//! struct Duplicator {
//! 	copy_a: String,
//! 	copy_b: String,
//! }
//!
//! impl From<String> for Duplicator {
//! 	fn from(value: String) -> Self {
//!         Self {
//! 			copy_a: value.clone(),
//! 			copy_b: value
//! 		}
//!     }
//! }
//!
//! micronfig::config! {
//! 	MY_CUSTOM_TYPE: String -> Duplicator,
//! }
//!
//! # fn main() {}
//! ```
//!
//! Conversions can be chained too:
//!
//! ```
//! struct ChatId(u64);
//!
//! impl From<u64> for ChatId {
//! 	fn from(value: u64) -> Self {
//!         Self(value)
//!     }
//! }
//!
//! micronfig::config! {
//! 	// First parse the string as an u64 with FromStr, then convert it to a ChatId with From.
//! 	RESPOND_TO_MESSAGES_IN: String > u64 -> ChatId,
//! }
//!
//! # fn main() {}
//! ```
//!
//! ## Crate features
//!
//! ### Value sources
//!
//! The crate supports retrieving values from various different sources depending on the needs of the application.
//!
//! The sources can be toggled on and off via crate features, and are listed in the following table in order of retrieval priority, where the topmost one is the first source that is checked, and the following ones are checked only if no value is detected in the ones above.
//!
//! | Feature | Description | Use case |
//! |---|---|---|
//! | `envfiles` | Contents of the file at the path indicated by the `{NAME}_FILE` environment variable. | Docker [configs](https://docs.docker.com/engine/swarm/configs/) and [secrets](https://docs.docker.com/engine/swarm/secrets/). |
//! | `envvars` | The `{NAME}` environment variable. | Most command-line applications. |
//! | `envdot` | The `.env` and `.env.local` files in the current working directory. | Application development. |
//!
//! By default, all of them are enabled.
//!

#![doc(html_logo_url = "https://raw.githubusercontent.com/Steffo99/micronfig/main/.media/icon-128x128_round.png")]

/// The macro described at the crate's root.
pub use micronfig_macros::config;

pub mod cache;

#[cfg(feature = "envvars")]
pub mod envvars;

#[cfg(feature = "envfiles")]
pub mod envfiles;

#[cfg(feature = "envdot")]
pub mod envdot;

#[cfg(test)]
pub mod testing;

