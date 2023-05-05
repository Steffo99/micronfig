//! Middle-level API — Use all available configuration sources.
//!
//! It can be useful if you want more control on how errors are handled or on how the key is passed to the [`crate::single`] sources.


use std::ffi::OsString;
#[cfg(feature = "single_envvars")] use crate::single::envvars;
#[cfg(feature = "single_envfiles")] use crate::single::envfiles;


/// Get a value from the first available source, additionally returning information about how the value was retrieved.
///
/// # Process
///
/// This function tries to `get` a configuration value:
///
/// 1. with [`envvars::get`], using `key`, returning a [`Source::EnvVar`]
/// 2. with [`envfiles::get`], using `key + key_suffix_file`, returning a [`Source::EnvFile`]
///
/// If none of these options successfully resulted in the successful retrieval of the configuration value, [`Source::NotFound`] is returned instead.
///
/// # Errors
///
/// All errors encountered are bubbled up, except the ones surfacing because of the total absence of a configuration value, which make the function immediately try the next available source.
///
/// Currently, those errors are:
/// - [`envvars::Error::CannotReadEnvVar`]
/// - [`envfiles::Error::CannotReadEnvVar`]
///
/// # Examples
///
/// Retrieve a configuration value from either the `USER` environment variable or the `USER_FILE` file, maintaining it as a [`String`]:
/// ```
/// use micronfig::multi::get;
/// use micronfig::multi::Source;
/// #
/// # std::env::set_var("USER", "steffo");
/// # std::env::remove_var("USER_FILE");
///
/// let user: Source<u32> = get("USER", "_FILE");
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS` environment variable or the `IP_ADDRESS_FILE` file, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// use micronfig::multi::get;
/// use micronfig::multi::Source;
/// #
/// # std::env::set_var("IP_ADDRESS", "192.168.1.1");
/// # std::env::remove_var("IP_ADDRESS_FILE");
///
/// let ip_addr: Source<IpAddr> = get("IP_ADDRESS", "_FILE");
/// ```
///
pub fn get<Key, KeySuffixFile, Type>(key: Key, key_suffix_file: KeySuffixFile) -> Source<Type>
    where Key: AsRef<std::ffi::OsStr>,
          KeySuffixFile: AsRef<std::ffi::OsStr>,
          Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{

    if cfg!(feature = "single_envvars") {
        let v = envvars::get(&key);

        match v {
            Err(envvars::Error::CannotReadEnvVar(_)) => {},
            _ => return Source::EnvVar(v),
        }
    }

    if cfg!(feature = "single_envfiles") {
        let mut key_file = OsString::new();
        key_file.push(key);
        key_file.push(key_suffix_file);
        let v = envfiles::get(key_file);

        match v {
            Err(envfiles::Error::CannotReadEnvVar(_)) => {},
            _ => return Source::EnvFile(v),
        }
    }

    Source::NotFound
}


/// The way the result returned by [`get`] was obtained.
///
/// Since more sources might be added in the future, this function is `non_exaustive`.
#[non_exhaustive]
#[derive(Debug)]
pub enum Source<Type>
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
{
    /// The result was not obtained, since the configuration value was not defined anywhere.
    NotFound,

    /// The result was obtained by [`envvars::get`].
    #[cfg(feature = "single_envvars")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "single_envvars")))]
    EnvVar(envvars::Result<Type>),

    /// The result was obtained by [`envfiles::get`].
    #[cfg(feature = "single_envfiles")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "single_envfiles")))]
    EnvFile(envfiles::Result<Type>),
}

impl<Type> Source<Type>
    where Type: std::str::FromStr,
          <Type as std::str::FromStr>::Err: std::fmt::Debug,
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
    /// use micronfig::multi::Source;
    ///
    /// let value = Source::<u8>::EnvFile(Ok(1)).expect("value to be present");
    /// assert_eq!(value, 1)
    /// ```
    ///
    /// ```should_panic
    /// use micronfig::multi::Source;
    /// use micronfig::single::envfiles::Error as FileError;
    ///
    /// let value = Source::<u8>::EnvFile(Err(FileError::CannotReadEnvVar(std::env::VarError::NotPresent))).expect("value to be present");
    /// // Panic!
    /// ```
    pub fn expect(self, msg: &str) -> Type {
        match self {
            #[cfg(feature = "single_envvars")]
            Self::EnvVar(Ok(v)) => v,

            #[cfg(feature = "single_envfiles")]
            Self::EnvFile(Ok(v)) => v,

            _ => panic!("{}", msg),
        }
    }

    /// Returns any contained [`Ok`] value, consuming both `self` and the [`Source`] inside.
    ///
    /// # Panics
    ///
    /// This function panics if `self` is a [`Source::NotFound`], or if the contained value is a [`Err`].
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
    /// use micronfig::multi::Source;
    ///
    /// let value = Source::<u8>::EnvFile(Ok(1)).unwrap();
    /// assert_eq!(value, 1)
    /// ```
    ///
    /// ```should_panic
    /// use micronfig::multi::Source;
    /// use micronfig::single::envfiles::Error as FileError;
    ///
    /// let value = Source::<u8>::EnvFile(Err(FileError::CannotReadEnvVar(std::env::VarError::NotPresent))).unwrap();
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
    use crate::testing::tempfile_fixture;

    #[test]
    #[cfg(feature = "single_envvars")]
    fn it_works_var() {
        std::env::set_var("NUMBER", "1");
        std::env::remove_var("NUMBER_FILE");

        match get::<&str, &str, u32>("NUMBER", "_FILE") {
            Source::EnvVar(Ok(1u32)) => {},
            _ => panic!("expected Source::EnvVar(Ok(1u32))")
        }
    }

    #[test]
    #[cfg(feature = "single_envfiles")]
    fn it_works_file() {
        let file = tempfile_fixture("1");
        std::env::remove_var("NUMBER");
        std::env::set_var("NUMBER_FILE", file.as_os_str());

        let n = get::<&str, &str, u32>("NUMBER", "_FILE");
        match n {
            Source::EnvFile(Ok(1u32)) => {},
            _ => panic!("expected Source::EnvFile(Ok(1u32))")
        }
    }

    #[test]
    #[cfg(feature = "single_envvars")]
    fn missing_envvar() {
        match get::<&str, &str, String>("MISSING_ENVVAR", "_FILE") {
            Source::NotFound => {},
            _ => panic!("expected Source::NotFound"),
        }
    }

    #[test]
    #[cfg(feature = "single_envfiles")]
    fn missing_file() {
        std::env::remove_var("NUMBER");
        std::env::set_var("NUMBER_FILE", "/this/file/does/not/exist");

        match get::<&str, &str, u32>("NUMBER", "_FILE") {
            Source::EnvFile(Err(envfiles::Error::CannotOpenFile(_))) => {},
            _ => panic!("expected Source::EnvFile(Err(envfiles::Error::CannotOpenFile(_)))"),
        }
    }

    #[test]
    #[cfg(feature = "single_envvars")]
    fn not_a_number_var() {
        std::env::set_var("NUMBER", "XYZ");
        std::env::remove_var("NUMBER_FILE");

        match get::<&str, &str, u32>("NUMBER", "_FILE") {
            Source::EnvVar(Err(envvars::Error::CannotConvertValue(_))) => {},
            _ => panic!("expected Source::EnvVar(Err(envvars::Error::CannotConvertValue(_)))"),
        }
    }

    #[test]
    #[cfg(feature = "single_envfiles")]
    fn not_a_number_file() {
        let file = tempfile_fixture("XYZ");
        std::env::set_var("NUMBER_FILE", file.as_os_str());
        std::env::remove_var("NUMBER");

        match get::<&str, &str, u32>("NUMBER", "_FILE") {
            Source::EnvFile(Err(envfiles::Error::CannotConvertValue(_))) => {},
            _ => panic!("expected Source::EnvFile(Err(envfiles::Error::CannotConvertValue(_)))"),
        }
    }

    #[test]
    #[cfg(feature = "single_envvars")]
    fn unwrap_var_ok() {
        Source::EnvVar(Ok("ok".to_string())).unwrap();
    }

    #[test]
    #[cfg(feature = "single_envfiles")]
    fn unwrap_file_ok() {
        Source::EnvFile(Ok("ok".to_string())).unwrap();
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "single_envvars")]
    fn unwrap_var_err() {
        Source::<String>::EnvVar(Err(envvars::Error::CannotReadEnvVar(std::env::VarError::NotPresent))).unwrap();
    }

    #[test]
    #[should_panic]
    #[cfg(feature = "single_envfiles")]
    fn unwrap_file_err() {
        Source::<String>::EnvFile(Err(envfiles::Error::CannotReadEnvVar(std::env::VarError::NotPresent))).unwrap();
    }
}