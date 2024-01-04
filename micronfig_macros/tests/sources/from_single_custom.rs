#[derive(std::fmt::Debug, Eq)]
struct MyCustomStruct(String);

impl From<String> for MyCustomStruct {
	fn from(value: String) -> Self {
		Self(value)
	}
}

micronfig::config! {
	GARASAUTO: String -> crate::MyCustomStruct,
}

fn main() {
	std::env::set_var("GARASAUTO", "baba");
	assert_eq!(GARASAUTO(), &MyCustomStruct("baba"));
}
