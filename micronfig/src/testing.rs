pub fn tempfile_fixture(content: &str) -> tempfile::TempPath {
	use std::io::Write;

	let mut file = tempfile::NamedTempFile::new()
		.expect("the tempfile fixture to be created successfully");
	write!(file, "{}", content)
		.expect("to be able to write into the tempfile fixture");

	file.into_temp_path()
}

