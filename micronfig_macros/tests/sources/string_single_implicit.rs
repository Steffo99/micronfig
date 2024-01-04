micronfig::config! {
	GARASAUTO,
}

fn main() {
	std::env::set_var("GARASAUTO", "fieraereo");
	assert_eq!(GARASAUTO(), "fieraereo");
}
