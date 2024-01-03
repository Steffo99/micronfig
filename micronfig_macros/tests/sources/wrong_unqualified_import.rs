use std::path::PathBuf;

micronfig::config! {
	GARASAUTO: String > PathBuf,
}

fn main() {
	println!("{:?}", GARASAUTO())
}
