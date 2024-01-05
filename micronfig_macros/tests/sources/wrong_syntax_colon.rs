micronfig::config! {
	GARASAUTO: ,
}

fn main() {
	std::env::set_var("GARASAUTO", "garasauto");
	println!("{:#?}", GARASAUTO());
}
