micronfig::config! {
	GARASAUTO: String > u128 => u64 => u32 => u16 => u8,
}

fn main() {
	std::env::set_var("GARASAUTO", "1");
	assert_eq!(GARASAUTO(), &1u8);
}
