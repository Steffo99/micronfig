//! High-level API — Handle errors automatically.
//!
//! It can be useful if you want to specify when configuration values are loaded in the lifecycle of your binary.


/// Get a value from the first available source, panicking with a human-readable message in case the value is missing or cannot be processed.
///
/// # Process
///
/// This function:
///
/// 1. calls [`crate::multi::get`] with the given key and a file suffix of `_FILE`
/// 2. pattern matches errors and [`panic`]s if an error is caught.
///
/// # Panics
///
/// Any error encountered by this function causes a panic with a message describing what went wrong.
///
/// The same thing happens if the configuration value could not be retrieved by any source.
///
/// # Examples
///
/// Retrieve a configuration value from either the `USER` environment variable or the `USER_FILE` file, maintaining it as a [`String`]:
/// ```
/// use micronfig::handle::get_required;
/// #
/// # std::env::set_var("USER", "steffo");
/// # std::env::remove_var("USER_FILE");
///
/// let user: String = get_required("USER");
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS` environment variable or the `IP_ADDRESS_FILE` file, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// use micronfig::handle::get_required;
/// #
/// # std::env::set_var("IP_ADDRESS", "192.168.1.1");
/// # std::env::remove_var("IP_ADDRESS_FILE");
///
/// let ip_addr: IpAddr = get_required("IP_ADDRESS");
/// ```
///
/// # See also
///
/// [`get_optional`], which has the same behaviour but does not panic if the value is not found, instead returning [`None`].
///
/// # Possible future improvements
///
/// Possibly refactor this to a method of [`crate::multi::Source`].
///
pub fn get_required<Type>(key: &str) -> Type
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    use crate::multi::{get, Source};
    use crate::single::{envvars, envfiles};

    match get(key, "_FILE") {
        Source::EnvVar(Ok(v)) => v,
        Source::EnvVar(Err(envvars::Error::CannotConvertValue(err))) =>
            panic!("The contents of the {} environment variable could not be converted to a {}: {:?}", &key, &std::any::type_name::<Type>(), &err),
        Source::EnvVar(Err(envvars::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::EnvFile(Ok(v)) => v,
        Source::EnvFile(Err(envfiles::Error::CannotConvertValue(err))) =>
            panic!("The contents of the file at {} could not be converted to a {}: {:?}", &key, &std::any::type_name::<Type>(), &err),
        Source::EnvFile(Err(envfiles::Error::CannotOpenFile(err))) =>
            panic!("The file at {} could not be opened: {}", &key, &err),
        Source::EnvFile(Err(envfiles::Error::CannotReadFile(err))) =>
            panic!("The contents of the file at {} could not be read: {}", &key, &err),
        Source::EnvFile(Err(envfiles::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::NotFound =>
            panic!("The configuration value {} is not defined.", &key),
    }
}


/// Try to get a value from the first available source, panicking with a human-readable message in case it cannot be processed.
///
/// # Process
///
/// This function:
///
/// 1. calls [`crate::multi::get`] with the given key and a file suffix of `_FILE`
/// 2. pattern matches errors and [`panic`]s if an error is caught.
///
/// # Panics
///
/// Any error encountered by this function causes a panic with a message describing what went wrong.
///
/// # Examples
///
/// Retrieve a configuration value from either the `USER` environment variable or the `USER_FILE` file, maintaining it as a [`String`]:
/// ```
/// use micronfig::handle::get_optional;
/// #
/// # std::env::set_var("USER", "steffo");
/// # std::env::remove_var("USER_FILE");
///
/// let user: Option<String> = get_optional("USER");
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS` environment variable or the `IP_ADDRESS_FILE` file, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// use micronfig::handle::get_optional;
/// #
/// # std::env::set_var("IP_ADDRESS", "192.168.1.1");
/// # std::env::remove_var("IP_ADDRESS_FILE");
///
/// let ip_addr: Option<IpAddr> = get_optional("IP_ADDRESS");
/// ```
///
/// # See also
///
/// [`get_required`], which has the same behaviour but does panics if the value is not found.
///
/// # Possible future improvements
///
/// Possibly refactor this to a method of [`crate::multi::Source`].
///
pub fn get_optional<Type>(name: &str) -> Option<Type>
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    use crate::multi::{get, Source};
    use crate::single::{envvars, envfiles};

    match get(name, "_FILE") {
        Source::EnvVar(Ok(v)) => Some(v),
        Source::EnvVar(Err(envvars::Error::CannotConvertValue(err))) =>
            panic!("The contents of the {} environment variable could not be converted to a {}: {:?}", &name, &std::any::type_name::<Type>(), &err),
        Source::EnvVar(Err(envvars::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::EnvFile(Ok(v)) => Some(v),
        Source::EnvFile(Err(envfiles::Error::CannotConvertValue(err))) =>
            panic!("The contents of the file at {} could not be converted to a {}: {:?}", &name, &std::any::type_name::<Type>(), &err),
        Source::EnvFile(Err(envfiles::Error::CannotOpenFile(err))) =>
            panic!("The file at {} could not be opened: {}", &name, &err),
        Source::EnvFile(Err(envfiles::Error::CannotReadFile(err))) =>
            panic!("The contents of the file at {} could not be read: {}", &name, &err),
        Source::EnvFile(Err(envfiles::Error::CannotReadEnvVar(_))) =>
            panic!("Something unexpected happened in micronfig. Please report this as a bug!"),

        Source::NotFound => None,

    }
}
