//! Module defining the [`get`] low-level function for environment files, and its [`Error`] and [`Result`] associated types.


/// Get a configuration value from the file at the path contained in the environment variable with the given `key`, and convert it to the desired `Type`.
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
    /// Encountered when the call to [`FromStr::from_str`] fails.
    CannotConvertValue(ConversionError),
}


/// A possible error encountered by [`get`].
pub type Result<Type> = std::result::Result<Type, Error<<Type as std::str::FromStr>::Err>>;


#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::tempfile_fixture;

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