//! Environment variables.


/// Get a configuration value from the source.
///
/// # Process
///
/// This function:
///
/// 1. tries to access the environment variable with the given name using [`std::env::var`]
/// 2. tries to convert the obtained value to another of the given type using [`std::str::FromStr::from_str`]
///
/// # Examples
///
/// Retrieve a configuration value from the `USER` environment variable, maintaining it as a [`String`]:
/// ```
/// use micronfig::single::envvars::get;
///
/// # std::env::set_var("USER", "steffo");
/// let user: String = get("USER").expect("USER envvar to be defined");
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS` environment variable, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// use micronfig::single::envvars::get;
///
/// # std::env::set_var("IP_ADDRESS", "192.168.1.1");
/// let ip_addr: IpAddr = get("IP_ADDRESS").expect("IP_ADDRESS envvar to be defined");
/// ```
///
pub fn get<Key, Type>(key: Key) -> Result<Type>
    where Key: AsRef<std::ffi::OsStr>,
          Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    let data = std::env::var(key)
        .map_err(Error::CannotReadEnvVar)?;

    let value = Type::from_str(&data)
        .map_err(Error::CannotConvertValue)?;

    Ok(value)
}


/// A possible error encountered by [`get`].
#[derive(std::fmt::Debug)]
pub enum Error<ConversionError>
    where ConversionError: std::fmt::Debug,
{
    /// The environment variable could not be read.
    ///
    /// Encountered when the call to [`std::env::var`] fails.
    CannotReadEnvVar(std::env::VarError),

    /// The value could not be converted to the desired type.
    ///
    /// Encountered when the call to [`std::str::FromStr::from_str`] fails.
    CannotConvertValue(ConversionError),
}


/// The result of [`get`].
pub type Result<Type> = std::result::Result<Type, Error<<Type as std::str::FromStr>::Err>>;


#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    #[test]
    fn it_works() {
        std::env::set_var("NUMBER", "1");

        let number = get::<&str, u32>("NUMBER").unwrap();
        assert_eq!(number, 1u32);
    }

    #[test]
    fn missing_envvar() {
        std::env::remove_var("THIS_ENVVAR_DOES_NOT_EXIST");

        match get::<&str, String>("THIS_ENVVAR_DOES_NOT_EXIST") {
            Err(Error::CannotReadEnvVar(std::env::VarError::NotPresent)) => {},
            _ => panic!("expected Err(Error::CannotReadEnvVar(std::env::VarError::NotPresent))"),
        }
    }

    #[test]
    fn not_a_number() {
        std::env::set_var("NUMBER", "XYZ");

        match get::<&str, u32>("NUMBER") {
            Err(Error::CannotConvertValue(_)) => {},
            _ => panic!("expected Error::CannotConvertValue(_)"),
        }
    }
}
