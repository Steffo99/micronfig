//! Module defining the general [`get`] low-level function and its associated [`Source`] type.

use crate::var;
use crate::file;


/// Get a configuration value, maintaining information about how the value was retrieved.
///
/// This function tries to get a configuration value:
///
/// 1. with [`var::get`] using `name_var`, returning a [`Source::Var`]
/// 2. with [`file::get`] using `name_file`, returning a [`Source::File`]
///
/// If none of these options successfully resulted in the successful retrieval of the configuration value, [`Source::NotFound`] is returned.
///
/// All errors are bubbled up, except the ones surfacing because of the total absence of a configuration value, currently:
/// - [`var::Error::CannotReadEnvVar`]
/// - [`file::Error::CannotReadEnvVar`]
///
/// # Examples
///
/// ```
/// use micronfig::any::get;
/// use micronfig::any::Source;
///
/// // The NUMBER envvar has been previously set to "1".
/// # std::env::set_var("NUMBER", "1");
/// # std::env::remove_var("NUMBER_FILE");
///
/// let value = get::<&str, &str, u32>("NUMBER", "NUMBER_FILE");
/// if let Source::Var(Ok(1)) = value {} else { panic!() }
/// ```
///
/// ```
/// use micronfig::any::get;
/// use micronfig::any::Source;
///
/// // The NUMBER and NUMBER_FILE envvars have not been set.
/// # std::env::remove_var("NUMBER");
/// # std::env::remove_var("NUMBER_FILE");
///
/// let value = get::<&str, &str, u32>("NUMBER", "NUMBER_FILE");
/// if let Source::NotFound = value {} else { panic!() }
/// ```
///
pub fn get<KeyVar, KeyFile, Type>(name_var: KeyVar, name_file: KeyFile) -> Source<Type>
    where KeyVar: AsRef<std::ffi::OsStr>,
          KeyFile: AsRef<std::ffi::OsStr>,
          Type: std::str::FromStr,
{
    let v = var::get(name_var);

    match v {
        Err(var::Error::CannotReadEnvVar(_)) => {},
        _ => return Source::Var(v),
    }

    let v = file::get(name_file);

    match v {
        Err(file::Error::CannotReadEnvVar(_)) => {},
        _ => return Source::File(v),
    }

    Source::NotFound
}


/// The way the result returned by [`get`] was obtained.
///
/// Since more sources might be added in the future, this function is `non_exaustive`.
#[non_exhaustive]
pub enum Source<Type>
    where Type: std::str::FromStr,
{
    /// The result was not obtained, since the configuration value was not defined anywhere.
    NotFound,

    /// The result was obtained by [`var::get`].
    Var(var::Result<Type>),

    /// The result was obtained by [`file::get`].
    File(file::Result<Type>),
}

impl<Type> Source<Type>
    where Type: std::str::FromStr,
{
    /// Returns any contained [`Ok`] value, consuming both `self` and the [`Source`] inside.
    ///
    /// # Panics
    ///
    /// This function panics if `self` is a [`Source::NotFound`], or if the contained value is a [`Err`].
    ///
    /// The panic message is the `msg` given.
    ///
    /// # See also
    ///
    /// Similar to [`Result::expect`].
    ///
    /// Used by [`Self::unwrap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use micronfig::any::Source;
    ///
    /// let value = Source::<u8>::File(Ok(1)).expect("value to be present");
    /// assert_eq!(value, 1)
    /// ```
    ///
    /// ```should_panic
    /// use micronfig::any::Source;
    /// use micronfig::file::Error as FileError;
    ///
    /// let value = Source::<u8>::File(Err(FileError::CannotReadEnvVar(std::env::VarError::NotPresent))).expect("value to be present");
    /// // Panic!
    /// ```
    pub fn expect(self, msg: &str) -> Type {
        match self {
            Self::Var(Ok(v)) => v,
            Self::File(Ok(v)) => v,
            _ => panic!("{}", msg),
        }
    }

    /// Returns any contained [`Ok`] value, consuming both `self` and the [`Source`] inside.
    ///
    /// # Panics
    ///
    /// This function panics if `self` is a [`Source::NotFound`], or if the contained value is a [`Err`].
    ///
    /// The panic message is the `msg` given.
    ///
    /// # See also
    ///
    /// Similar to [`Result::unwrap`].
    ///
    /// Internally, it uses [`Self::expect`].
    ///
    /// # Examples
    ///
    /// ```
    /// use micronfig::any::Source;
    ///
    /// let value = Source::<u8>::File(Ok(1)).unwrap();
    /// assert_eq!(value, 1)
    /// ```
    ///
    /// ```should_panic
    /// use micronfig::any::Source;
    /// use micronfig::file::Error as FileError;
    ///
    /// let value = Source::<u8>::File(Err(FileError::CannotReadEnvVar(std::env::VarError::NotPresent))).unwrap();
    /// // Panic!
    /// ```
    pub fn unwrap(self) -> Type
    {
        self.expect("called `Source::unwrap()` on an invalid variant, such as `NotFound` or `_(Err(_))`")
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::tempfile_fixture;

    #[test]
    fn it_works_var() {
        std::env::set_var("NUMBER", "1");
        std::env::remove_var("NUMBER_FILE");

        match get::<&str, &str, u32>("NUMBER", "NUMBER_FILE") {
            Source::Var(Ok(1u32)) => {},
            _ => panic!("expected Source::Var(Ok(1u32))")
        }
    }

    #[test]
    fn it_works_file() {
        let file = tempfile_fixture("1");
        std::env::remove_var("NUMBER");
        std::env::set_var("NUMBER_FILE", file.as_os_str());

        let n = get::<&str, &str, u32>("NUMBER", "NUMBER_FILE");
        match n {
            Source::File(Ok(1u32)) => {},
            _ => panic!("expected Source::File(Ok(1u32))")
        }
    }

    #[test]
    fn missing_envvar() {
        match get::<&str, &str, String>("MISSING_ENVVAR", "MISSING_ENVVAR_FILE") {
            Source::NotFound => {},
            _ => panic!("expected Source::NotFound"),
        }
    }

    #[test]
    fn missing_file() {
        std::env::remove_var("NUMBER");
        std::env::set_var("NUMBER_FILE", "/this/file/does/not/exist");

        match get::<&str, &str, u32>("NUMBER", "NUMBER_FILE") {
            Source::File(Err(file::Error::CannotOpenFile(_))) => {},
            _ => panic!("expected Source::File(Err(file::Error::CannotOpenFile(_)))"),
        }
    }

    #[test]
    fn not_a_number_var() {
        std::env::set_var("NUMBER", "XYZ");
        std::env::remove_var("NUMBER_FILE");

        match get::<&str, &str, u32>("NUMBER", "NUMBER_FILE") {
            Source::Var(Err(var::Error::CannotConvertValue(_))) => {},
            _ => panic!("expected Source::Var(Err(var::Error::CannotConvertValue(_)))"),
        }
    }

    #[test]
    fn not_a_number_file() {
        let file = tempfile_fixture("XYZ");
        std::env::set_var("NUMBER_FILE", file.as_os_str());
        std::env::remove_var("NUMBER");

        match get::<&str, &str, u32>("NUMBER", "NUMBER_FILE") {
            Source::File(Err(file::Error::CannotConvertValue(_))) => {},
            _ => panic!("expected Source::File(Err(file::Error::CannotConvertValue(_)))"),
        }
    }

    #[test]
    fn unwrap_var_ok() {
        Source::Var(Ok("ok".to_string())).unwrap();
    }

    #[test]
    fn unwrap_file_ok() {
        Source::File(Ok("ok".to_string())).unwrap();
    }

    #[test]
    #[should_panic]
    fn unwrap_var_err() {
        Source::<String>::Var(Err(var::Error::CannotReadEnvVar(std::env::VarError::NotPresent))).unwrap();
    }

    #[test]
    #[should_panic]
    fn unwrap_file_err() {
        Source::<String>::File(Err(file::Error::CannotReadEnvVar(std::env::VarError::NotPresent))).unwrap();
    }
}