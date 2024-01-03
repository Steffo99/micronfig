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
pass!(string_single_implicit);
pass!(string_multi_explicit);
pass!(string_multi_implicit);
pass!(string_multi_mixed);
pass!(parse_single_u64);
pass!(parse_single_i64);
pass!(parse_single_ipaddr);
pass!(parse_single_pathbuf_full);
pass!(parse_single_pathbuf_use);
