micronfig::config! {
	GARASAUTO: String,
}

fn main() {
	std::env::set_var("GARASAUTO", "sagramoto");
	assert_eq!(GARASAUTO(), "sagramoto");
}
