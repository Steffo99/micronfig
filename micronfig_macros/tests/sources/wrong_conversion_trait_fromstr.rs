micronfig::config! {
	GARASAUTO: String > std::convert::Infallible,
}

fn main() {
	println!("{:?}", GARASAUTO())
}
