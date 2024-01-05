micronfig::config! {
	GARASAUTO?: String > u64,
}

fn main() {
	std::env::remove_var("GARASAUTO");
	assert_eq!(GARASAUTO(), &None);
}
