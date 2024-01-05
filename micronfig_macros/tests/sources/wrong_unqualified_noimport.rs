micronfig::config! {
	GARASAUTO: String > PathBuf,
}

fn main() {
	std::env::set_var("GARASAUTO", "./bus");
	println!("{:#?}", GARASAUTO());
}
