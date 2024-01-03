macro_rules! pass {
	($id:ident) => {
		#[test]
		fn $id() {
			trybuild::TestCases::new().pass(format!("tests/sources/{}.rs", stringify!($id)));
		}
	}
}

macro_rules! fail {
	($id:ident) => {
		#[test]
		fn $id() {
			trybuild::TestCases::new().compile_fail(format!("tests/sources/{}.rs", stringify!($id)));
		}
	}
}

pass!(empty);
pass!(string_single_explicit);
