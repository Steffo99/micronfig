micronfig::config! {
	GARAS: String,
	AUTO: String,
	BUS: String,
}

fn main() {
	std::env::set_var("GARAS", "garas");
	std::env::set_var("AUTO", "auto");
	std::env::set_var("BUS", "bus");
	assert_eq!(GARAS(), "garas");
	assert_eq!(AUTO(), "auto");
	assert_eq!(BUS(), "bus");
}
