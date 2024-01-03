micronfig::config! {
	GARAS: String,
	AUTO: String,
	BUS: String,
}

fn main() {
	println!("{:?} {:?} {:?}", GARAS(), AUTO(), BUS())
}
