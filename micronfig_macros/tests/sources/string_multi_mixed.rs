micronfig::config! {
	GARAS,
	AUTO: String,
	BUS,
}

fn main() {
	println!("{:?} {:?} {:?}", GARAS(), AUTO(), BUS())
}
