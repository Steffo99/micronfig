#[test]
fn trybuild() {
	let t = trybuild::TestCases::new();
	t.pass("tests/configs/*.rs");
}
