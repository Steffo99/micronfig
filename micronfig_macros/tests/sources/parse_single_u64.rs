micronfig::config! {
	GARASAUTO: String > u64,
}

fn main() {
	std::env::set_var("GARASAUTO", "1");
	assert_eq!(GARASAUTO(), &1u64);
}
