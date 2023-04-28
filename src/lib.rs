//! Tiny crate for [twelve-factor app configuration](https://12factor.net/config).
//!
//! # Goals
//!
//! This crate aims to simplify developing and deploying Docker-compatible services in Rust.
//!
//! # Features
//!
//! This crate handles:
//!
//! 1. Retrieval of configuration values from multiple sources
//!     1. The environment
//!     2. Files specified in the environment
//! 2. Conversion to a value of an arbitrary type
//! 3. Displaying a operator-friendly error if case one of this steps did not succeed

pub mod any;
pub mod var;
pub mod file;

/// Get the configuration value with the given `name` and convert it to the given `Type`.
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
pub fn required<Type>(name: &str) -> Type
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    use crate::any::{get, Source};

    match get(name, "_FILE") {
        Source::Var(Ok(v)) => v,
        Source::Var(Err(var::Error::CannotConvertValue(err))) =>
            panic!("The contents of the {} environment variable could not be converted to a {}: {:?}", &name, &std::any::type_name::<Type>(), &err),
        Source::Var(Err(var::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::File(Ok(v)) => v,
        Source::File(Err(file::Error::CannotConvertValue(err))) =>
            panic!("The contents of the file at {} could not be converted to a {}: {:?}", &name, &std::any::type_name::<Type>(), &err),
        Source::File(Err(file::Error::CannotOpenFile(err))) =>
            panic!("The file at {} could not be opened: {}", &name, &err),
        Source::File(Err(file::Error::CannotReadFile(err))) =>
            panic!("The contents of the file at {} could not be read: {}", &name, &err),
        Source::File(Err(file::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::NotFound =>
            panic!("The configuration value {} is not defined.", &name),
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