micronfig::config! {
	GARASAUTO: String > std::path::PathBuf,
}

fn main() {
	std::env::set_var("GARASAUTO", "./garas");
	assert_eq!(GARASAUTO(), std::path::PathBuf::from("./garas"));
}
