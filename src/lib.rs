use std::env;
use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use std::fs::File;

/// An error that occurred while getting a configuration value.
pub enum GetFileError<TargetType> where TargetType: TryFrom<String> {
    CannotReadEnvVar(env::VarError),
    CannotOpenFile(std::io::Error),
    CannotReadFile(std::io::Error),
    CannotConvertValue(<TargetType as TryFrom<String>>::Error),
}

/// The result of an attempt at getting a configuration value.
pub type GetFileResult<TargetType> = Result<TargetType, GetFileError<TargetType>>;

/// Get a configuration value from a file specified in the given environment variable.
/// 
///
/// # Parameters
///
/// - `name`: the name of the environment variable containing the location of the file containing the configuration value.
/// - `TargetType`: the struct the contents of the file are to be converted into using [`TryFrom`] [`String`].
/// 
pub fn get_file<TargetType>(name: &str) -> GetFileResult<TargetType> where TargetType: TryFrom<String> {
    let path = env::var(name).map_err(GetFileError::CannotReadEnvVar)?;
    let path = OsString::from(path);
    let path = PathBuf::from(path);

    let mut file = File::open(path).map_err(GetFileError::CannotOpenFile)?;
    
    let mut data = String::new();
    file.read_to_string(&mut data).map_err(GetFileError::CannotReadFile)?;

    let value = TargetType::try_from(data).map_err(GetFileError::CannotConvertValue)?;

    Ok(value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
