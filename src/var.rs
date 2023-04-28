//! Module defining a function retrieving a configuration value from the environment.


/// Get a value of the requested type from the environment variable with the given name.
pub fn get<Key, Type>(name: Key) -> Result<Type>
    where Key: AsRef<std::ffi::OsStr>,
          Type: std::str::FromStr,
{
    let data = std::env::var(name)
        .map_err(Error::CannotReadEnvVar)?;

    let value = Type::from_str(&data)
        .map_err(Error::CannotConvertValue)?;

    Ok(value)
}


/// A possible error encountered by [`get`].
#[derive(Debug)]
pub enum Error<ConversionError> {
    /// The environment variable could not be read.
    CannotReadEnvVar(std::env::VarError),

    /// The value could not be converted to the desired type.
    CannotConvertValue(ConversionError),
}


/// The result of [`get`].
pub type Result<Type> = std::result::Result<Type, Error<<Type as std::str::FromStr>::Err>>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        std::env::set_var("NUMBER", "1");

        let number = get::<&str, u32>("NUMBER").unwrap();
        assert_eq!(number, 1u32);
    }

    #[test]
    fn missing_envvar() {
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
