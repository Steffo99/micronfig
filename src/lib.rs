//! A tiny crate for [twelve-factor app configuration](https://12factor.net/config).

pub mod var;
pub mod file;


/// Get a value of the requested type, trying the following sources in order:
///
/// 1. the environment variable `{name}` (see [`var::get`])
/// 2. the contents of the file at the path specified at the environment variable `{name}_FILE` (see [`file::get`])
///
pub fn get<Type>(name: &str) -> Source<Type>
    where Type: std::str::FromStr,
{
    let v = var::get(name);

    match v {
        Err(var::Error::CannotReadEnvVar(_)) => {},
        _ => return Source::Var(v),
    }

    let v = file::get(format!("{name}_FILE"));

    match v {
        Err(file::Error::CannotReadEnvVar(_)) => {},
        _ => return Source::File(v),
    }

    Source::NotFound
}


#[non_exhaustive]
pub enum Source<Type>
    where Type: std::str::FromStr,
{
    Var(var::Result<Type>),
    File(file::Result<Type>),
    NotFound,
}

impl<Type> Source<Type>
    where Type: std::str::FromStr,
{
    /// Like [`Result::expect`], but tries to access the nested [`Ok`] value.
    pub fn expect(self, msg: &str) -> Type {
        match self {
            Self::Var(Ok(v)) => v,
            Self::File(Ok(v)) => v,
            _ => panic!("{}", msg),
        }
    }

    /// Like [`Result::unwrap`], but tries to access the nested [`Ok`] value.
    pub fn unwrap(self) -> Type {
        self.expect("called `Source::unwrap()` on an `Err` or `NotFound` value")
    }
}


#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn tempfile_fixture(content: &str) -> tempfile::TempPath {
        use std::io::Write;

        let mut file = tempfile::NamedTempFile::new()
            .expect("the tempfile fixture to be created successfully");
        write!(file, "{}", content)
            .expect("to be able to write into the tempfile fixture");

        file.into_temp_path()
    }

    #[test]
    fn it_works_var() {
        std::env::set_var("NUMBER", "1");
        std::env::remove_var("NUMBER_FILE");

        match get::<u32>("NUMBER") {
            Source::Var(Ok(1u32)) => {},
            _ => panic!("expected Source::Var(Ok(1u32))")
        }
    }

    #[test]
    fn it_works_file() {
        let file = tempfile_fixture("1");
        std::env::remove_var("NUMBER");
        std::env::set_var("NUMBER_FILE", file.as_os_str());

        let n = get::<u32>("NUMBER");
        match n {
            Source::File(Ok(1u32)) => {},
            _ => panic!("expected Source::File(Ok(1u32))")
        }
    }

    #[test]
    fn missing_envvar() {
        match get::<String>("THIS_ENVVAR_DOES_NOT_EXIST") {
            Source::NotFound => {},
            _ => panic!("expected Source::NotFound"),
        }
    }

    #[test]
    fn missing_file() {
        std::env::remove_var("NUMBER");
        std::env::set_var("NUMBER_FILE", "/this/file/does/not/exist");

        match get::<u32>("NUMBER") {
            Source::File(Err(file::Error::CannotOpenFile(_))) => {},
            _ => panic!("expected Source::File(Err(file::Error::CannotOpenFile(_)))"),
        }
    }

    #[test]
    fn not_a_number_var() {
        std::env::set_var("NUMBER", "XYZ");
        std::env::remove_var("NUMBER_FILE");

        match get::<u32>("NUMBER") {
            Source::Var(Err(var::Error::CannotConvertValue(_))) => {},
            _ => panic!("expected Source::Var(Err(var::Error::CannotConvertValue(_)))"),
        }
    }

    #[test]
    fn not_a_number_file() {
        let file = tempfile_fixture("XYZ");
        std::env::set_var("NUMBER_FILE", file.as_os_str());
        std::env::remove_var("NUMBER");

        match get::<u32>("NUMBER") {
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