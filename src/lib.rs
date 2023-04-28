//! A tiny crate for [twelve-factor app configuration](https://12factor.net/config).
//!
//! # Goals
//!
//! This crate aims to simplify developing and deploying Docker-compatible services in Rust.
//!
//! # Features
//!
//! This crate handles:
//!
//! 1. Retrieval of configuration values from multiple sources ([`any::get`])
//!     1. The environment ([`var::get`])
//!     2. Files specified in the environment ([`file::get`])
//! 2. Conversion to a value of an arbitrary type ([`std::str::FromStr`])
//! 3. Displaying a operator-friendly error in case
//!
//! # Usage
//!
//! The following example:
//!
//! 1. Tries to retrieve the value of the configuration value `THIS_ENVVAR_CONTAINS_ONE`
//!     1. From the `THIS_ENVVAR_CONTAINS_ONE` environment variable
//!     2. From the contents of the file specified in the `THIS_ENVVAR_CONTAINS_ONE_FILE` environment variable
//! 2. It converts the value to a [`u8`]
//! 3. Panics with a operator-friendly error if any of these steps failed
//!
//! ```
//! todo!()
//! ```

pub mod any;
pub mod var;
pub mod file;


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