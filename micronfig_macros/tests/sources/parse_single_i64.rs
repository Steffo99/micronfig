micronfig::config! {
	GARASAUTO: String > i64,
}

fn main() {
	std::env::set_var("GARASAUTO", "-1");
	assert_eq!(GARASAUTO(), &(-1i64));
}
