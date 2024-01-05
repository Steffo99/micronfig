micronfig::config! {
	GARASAUTO: String > u8 -> u16 -> u32 -> u64 -> u128,
}

fn main() {
	std::env::set_var("GARASAUTO", "1");
	assert_eq!(GARASAUTO(), &1u128);
}
