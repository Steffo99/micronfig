micronfig::config! {
	GARASAUTO: String > u8 -> u16 -> u32 -> u64 -> u128,
}

fn main() {
	println!("{:?}", GARASAUTO())
}
