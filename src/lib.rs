//! Tiny crate for simple configuration management.
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
//! Each configurable property of the dependent binary must have an arbitrary *key*, a name used to define its value, usually in `SCREAMING_SNAKE_CASE`.
//!
//! For example, some keys may be:
//!
//! - `TELEGRAM_API_KEY`
//! - `OAUTH2_CLIENT_SECRET`
//! - `SCREEN_RESOLUTION`
//!
//! ## High-level API
//!
//! The recommended usage of this crate is via the high-level API, which comprises the [`required`] and [`optional`] functions.
//!
//! They automatically try to retrieve a value from the following sources, in this order, returning as soon as one is found:
//!
//! 1. the contents of the environment variable `{key}`;
//! 2. the contents of the file located at path specified in the environment variable `{key}_FILE`.
//!
//! If no value is found, or if an error occurred while trying to retrieve it, the function panics with a human-readable error message.
//!
//! Additionally, they try to parse the value into the requested Rust type using its [`std::str::FromStr`] trait.
//!
//! If the conversion fails, the function panics, again providing a human-readable error message.
//!
//! ### Examples
//!
//! To require a `IP_ADDRESS` property to be configured, and to parse it as an [`std::net::IpAddr`], you may write the following code:
//!
//! ```
//! use std::net::IpAddr;
//!
//! # std::env::set_var("IP_ADDRESS", "192.168.1.1");
//! let ip_addr: IpAddr = micronfig::required("IP_ADDRESS");
//! ```
//!
//! To allow the user to not specify it, and provide a default, you may write:
//!
//! ```
//! use std::net::{IpAddr, Ipv4Addr};
//!
//! # std::env::remove_var("IP_ADDRESS");
//! let ip_addr: IpAddr = micronfig::optional("IP_ADDRESS").unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
//! ```
//!
//! ## Middle-level API
//!
//! If you want more control on how errors are handled or on how the key is manipulated to access values, you can use the middle-level API, comprised of the [`any::get`] function and the [`any::Source`] enum.
//!
//! [`any::get`] works similarly to the [`required`] and [`optional`] functions, but returns a [`any::Source`] enum variant instead, which denotes the source a result was obtained from, and contains the raw [`Result`] of the operation.
//!
//! ### Example
//!
//! To customize the handling of the same `IP_ADDRESS` as earlier, so that something is printed instead of the binary panicking, you may write the following code:
//!
//! ```
//! use std::net::IpAddr;
//! use micronfig::any::{get, Source};
//!
//! let ip_addr: Source<IpAddr> = get("IP_ADDRESS", "_FILE");
//!
//! match ip_addr {
//!     Source::Var(Ok(addr)) | Source::File(Ok(addr)) => println!("Success! · {}", &addr),
//!     _ => println!("Failure..."),
//! }
//! ```
//!
//! ## Low-level API
//!
//! Finally, if you want to override the accessed sources, you may use the low level API directly, comprised of the following modules:
//!
//! - [`var`] for accessing environment variable
//! - [`file`] for accessing files with the path defined in environment variables
//!
//! ### Example
//!
//! To retrieve the `IP_ADDRESS` only from the environment variable, and ignoring other sources:
//!
//! ```
//! use std::net::IpAddr;
//! use micronfig::var::get;
//!
//! # std::env::set_var("IP_ADDRESS", "192.168.1.1");
//! let ip_addr: IpAddr = get("IP_ADDRESS").expect("IP_ADDRESS envvar to be defined");
//! ```
//!
//! # More examples
//!
//! Other examples are provided in the crate source, [inside the `examples/` directory](https://github.com/Steffo99/micronfig/tree/main/examples).

pub mod any;
pub mod var;
pub mod file;

/// Get the configuration value with the given `key` and convert it to the given `Type`.
///
/// # Panics
///
/// Any error encountered by this function causes a panic with a message describing what went wrong.
///
/// The same thing happens if the configuration value could not be retrieved by any source.
///
/// # Examples
///
/// ```
/// // The NUMBER envvar has been previously set to "1".
/// # std::env::set_var("NUMBER", "1");
/// # std::env::remove_var("NUMBER_FILE");
///
/// let value: u8 = micronfig::required("NUMBER");
/// assert_eq!(value, 1u8);
/// ```
///
/// ```should_panic
/// // The NUMBER envvar has not been set.
/// # std::env::remove_var("NUMBER");
/// # std::env::remove_var("NUMBER_FILE");
///
/// let value: u8 = micronfig::required("NUMBER");
/// // Panic: The configuration value NUMBER is not defined.
/// ```
///
/// # See also
///
/// [`any::get`], the function called by this one to get the configuration value.
///
pub fn required<Type>(key: &str) -> Type
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    use crate::any::{get, Source};

    match get(key, "_FILE") {
        Source::Var(Ok(v)) => v,
        Source::Var(Err(var::Error::CannotConvertValue(err))) =>
            panic!("The contents of the {} environment variable could not be converted to a {}: {:?}", &key, &std::any::type_name::<Type>(), &err),
        Source::Var(Err(var::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::File(Ok(v)) => v,
        Source::File(Err(file::Error::CannotConvertValue(err))) =>
            panic!("The contents of the file at {} could not be converted to a {}: {:?}", &key, &std::any::type_name::<Type>(), &err),
        Source::File(Err(file::Error::CannotOpenFile(err))) =>
            panic!("The file at {} could not be opened: {}", &key, &err),
        Source::File(Err(file::Error::CannotReadFile(err))) =>
            panic!("The contents of the file at {} could not be read: {}", &key, &err),
        Source::File(Err(file::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::NotFound =>
            panic!("The configuration value {} is not defined.", &key),
    }
}


/// Get the configuration value with the given `name` and convert it to the given `Type`, if it was defined somewhere.
///
/// # Panics
///
/// Any error encountered by this function causes a panic with a message describing what went wrong.
///
/// # Examples
///
/// ```
/// // The NUMBER envvar has been previously set to "1".
/// # std::env::set_var("NUMBER", "1");
/// # std::env::remove_var("NUMBER_FILE");
///
/// let value: Option<u8> = micronfig::optional("NUMBER");
/// assert_eq!(value, Some(1u8));
/// ```
///
/// ```
/// // The NUMBER envvar has not been set.
/// # std::env::remove_var("NUMBER");
/// # std::env::remove_var("NUMBER_FILE");
///
/// let value: Option<u8> = micronfig::optional("NUMBER");
/// assert_eq!(value, None);
/// ```
///
/// # See also
///
/// [`any::get`], the function called by this one to get the configuration value.
///
pub fn optional<Type>(name: &str) -> Option<Type>
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    use crate::any::{get, Source};

    match get(name, "_FILE") {
        Source::Var(Ok(v)) => Some(v),
        Source::Var(Err(var::Error::CannotConvertValue(err))) =>
            panic!("The contents of the {} environment variable could not be converted to a {}: {:?}", &name, &std::any::type_name::<Type>(), &err),
        Source::Var(Err(var::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::File(Ok(v)) => Some(v),
        Source::File(Err(file::Error::CannotConvertValue(err))) =>
            panic!("The contents of the file at {} could not be converted to a {}: {:?}", &name, &std::any::type_name::<Type>(), &err),
        Source::File(Err(file::Error::CannotOpenFile(err))) =>
            panic!("The file at {} could not be opened: {}", &name, &err),
        Source::File(Err(file::Error::CannotReadFile(err))) =>
            panic!("The contents of the file at {} could not be read: {}", &name, &err),
        Source::File(Err(file::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::NotFound => None,

    }
}


#[cfg(test)]
pub(crate) mod tests {
    /// Create a temporary file and write `content` inside it.
    ///
    /// The file will be deleted as soon as the [`tempfile::TempPath`] is dropped.
    pub(crate) fn tempfile_fixture(content: &str) -> tempfile::TempPath {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new()
            .expect("the tempfile fixture to be created successfully");
        write!(file, "{}", content)
            .expect("to be able to write into the tempfile fixture");

        file.into_temp_path()
    }
}