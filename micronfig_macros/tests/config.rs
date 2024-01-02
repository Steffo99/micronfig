use micronfig_macros::config;

#[test]
fn basic() {
	config! {
		GARAS: String,
		AUTO: String,
		BUS: String,
	}
}

#[test]
fn empty() {
	config! {}
}

#[test]
fn conversion_simple() {
	config! {
		GARAS: String > u32,
		AUTO: String > u16,
	}
}

/*
#[test]
fn implicit() {
	config! {
		GARAS,
		AUTO,
	}
}

#[test]
fn conversion_implicit() {
	config! {
		GARAS: > u32,
		AUTO > u16,
	}
}
*/