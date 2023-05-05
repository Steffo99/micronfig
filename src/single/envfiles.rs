//! Contents of files at paths defined by environment variables.


/// Get a configuration value from the source.
///
/// # Process
///
/// This function:
///
/// 1. tries to access the environment variable with the given name using [`std::env::var`]
/// 2. tries to interpret the contents of the environment variable as a [`std::path::PathBuf`]
/// 3. tries to [`std::fs::File::open`] the file at that path
/// 4. tries to [`std::io::Read::read_to_string`] the contents of the opened file
/// 5. tries to convert the obtained value to another of the given type using [`std::str::FromStr::from_str`]
///
/// # Examples
///
/// Retrieve a configuration value from the `USER_FILE` file, maintaining it as a [`String`]:
/// ```
/// use micronfig::single::envfiles::get;
///
/// # let filename = micronfig::testing::tempfile_fixture("steffo");
/// # std::env::set_var("USER_FILE", filename.as_os_str());
/// let user: String = get("USER_FILE").expect("USER_FILE envvar to be defined");
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS_FILE` file, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// use micronfig::single::envfiles::get;
///
/// # let filename = micronfig::testing::tempfile_fixture("192.168.1.1");
/// # std::env::set_var("IP_ADDRESS_FILE", filename.as_os_str());
/// let ip_addr: IpAddr = get("IP_ADDRESS_FILE").expect("IP_ADDRESS_FILE envvar to be defined");
/// ```
///
pub fn get<Key, Type>(key: Key) -> Result<Type>
    where Key: AsRef<std::ffi::OsStr>,
          Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    let path = std::env::var(key)
        .map_err(Error::CannotReadEnvVar)?;
    let path = std::ffi::OsString::from(path);
    let path = std::path::PathBuf::from(path);

    let mut file = std::fs::File::open(path)
        .map_err(Error::CannotOpenFile)?;

    use std::io::Read;
    let mut data = String::new();
    file.read_to_string(&mut data)
        .map_err(Error::CannotReadFile)?;

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

    /// The specified file could not be opened. (Probably it doesn't exist.)
    ///
    /// Encountered when the call to [`std::fs::File::open`] fails.
    CannotOpenFile(std::io::Error),

    /// The specified file could not be read.
    ///
    /// Encountered when the call to [`std::io::Read::read_to_string`] fails.
    CannotReadFile(std::io::Error),

    /// The value could not be converted to the desired type.
    ///
    /// Encountered when the call to [`std::str::FromStr::from_str`] fails.
    CannotConvertValue(ConversionError),
}


/// A possible error encountered by [`get`].
pub type Result<Type> = std::result::Result<Type, Error<<Type as std::str::FromStr>::Err>>;


#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::testing::tempfile_fixture;

    #[test]
    fn it_works() {
        let file = tempfile_fixture("1");
        std::env::set_var("NUMBER_FILE", file.as_os_str());

        let number = get::<&str, u32>("NUMBER_FILE").unwrap();
        assert_eq!(number, 1u32);
    }

    #[test]
    fn missing_envvar() {
        std::env::remove_var("THIS_ENVVAR_DOES_NOT_EXIST_FILE");

        match get::<&str, String>("THIS_ENVVAR_DOES_NOT_EXIST_FILE") {
            Err(Error::CannotReadEnvVar(std::env::VarError::NotPresent)) => {},
            _ => panic!("expected Err(Error::CannotReadEnvVar(std::env::VarError::NotPresent))"),
        }
    }

    #[test]
    fn missing_file() {
        std::env::set_var("NUMBER_FILE", "/this/file/does/not/exist");

        match get::<&str, u32>("NUMBER_FILE") {
            Err(Error::CannotOpenFile(_)) => {},
            _ => panic!("expected Err(Error::CannotOpenFile(_))"),
        }
    }

    #[test]
    fn not_a_number() {
        let file = tempfile_fixture("XYZ");
        std::env::set_var("NUMBER_FILE", file.as_os_str());

        match get::<&str, u32>("NUMBER_FILE") {
            Err(Error::CannotConvertValue(_)) => {},
            _ => panic!("expected Err(Error::CannotConvertValue(_))"),
        }
    }
}