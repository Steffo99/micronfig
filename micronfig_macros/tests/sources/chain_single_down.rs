micronfig::config! {
	GARASAUTO: String > u128 => u64 => u32 => u16 => u8,
}

fn main() {
	println!("{:?}", GARASAUTO())
}
