micronfig::config! {
	GARASAUTO: String > std::path::PathBuf,
}

fn main() {
	println!("{:?}", GARASAUTO())
}
