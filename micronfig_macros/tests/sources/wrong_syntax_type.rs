micronfig::config! {
	GARASAUTO String,
}

fn main() {
	std::env::set_var("GARASAUTO", "garasauto");
	println!("{:#?}", GARASAUTO());
}
