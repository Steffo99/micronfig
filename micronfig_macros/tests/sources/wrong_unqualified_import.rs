use std::path::PathBuf;

micronfig::config! {
	GARASAUTO: String > PathBuf,
}

fn main() {
	std::env::set_var("GARASAUTO", "./auto");
	println!("{:#?}", GARASAUTO());
}
