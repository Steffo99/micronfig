micronfig::config! {
	GARASAUTO: i64,
}

fn main() {
	std::env::set_var("GARASAUTO", "-1");
	println!("{:#?}", GARASAUTO());
}
