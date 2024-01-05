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

pass!(chain_single_down);
pass!(chain_single_up);
pass!(empty);
pass!(example_angybot);
pass!(example_distributedarcade);
pass!(example_patchedporobot);
pass!(from_single_custom);
pass!(parse_single_custom);
pass!(parse_single_i64);
pass!(parse_single_pathbuf);
pass!(parse_single_u64);
pass!(parse_single_u64_optional);
pass!(string_multi_explicit);
pass!(string_multi_implicit);
pass!(string_multi_mixed);
pass!(string_single_explicit);
pass!(string_single_implicit);
pass!(tryfrom_single_custom);

fail!(wrong_conversion_longfatarrow);
fail!(wrong_conversion_longthinarrow);
fail!(wrong_conversion_tildearrow);
fail!(wrong_conversion_trait_from);
fail!(wrong_conversion_trait_fromstr);
fail!(wrong_conversion_trait_tryfrom);
fail!(wrong_nonsense_1);
fail!(wrong_nonsense_2);
fail!(wrong_nonsense_3);
fail!(wrong_start);
fail!(wrong_syntax_colon);
fail!(wrong_syntax_type);
fail!(wrong_unqualified_import);
fail!(wrong_unqualified_noimport);
