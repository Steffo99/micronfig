micronfig::config! {
	GARASAUTO: String ==> u64,
}

fn main() {
	std::env::set_var("GARASAUTO", "1");
	println!("{:#?}", GARASAUTO());
}
