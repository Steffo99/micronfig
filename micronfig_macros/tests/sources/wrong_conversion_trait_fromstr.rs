micronfig::config! {
	GARASAUTO: String > std::convert::Infallible,
}

fn main() {
	std::env::set_var("GARASAUTO", "!");
	println!("{:#?}", GARASAUTO());
}
