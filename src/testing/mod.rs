//! Fixtures for testing.
//!
//! **Unstable**; not supposed to be used outside this crate; do not add `pub(crate)` or doctests will stop working.

/// Create a temporary file and write `content` inside it.
///
/// The file will be deleted as soon as the [`tempfile::TempPath`] is dropped.
pub fn tempfile_fixture(content: &str) -> tempfile::TempPath {
    use std::io::Write;

    let mut file = tempfile::NamedTempFile::new()
        .expect("the tempfile fixture to be created successfully");
    write!(file, "{}", content)
        .expect("to be able to write into the tempfile fixture");

    file.into_temp_path()
}
