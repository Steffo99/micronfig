#[derive(std::fmt::Debug, Eq)]
struct MyCustomStruct(String);

impl std::convert::TryFrom<String> for MyCustomStruct {
	type Error = ();

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Ok(Self(value))
	}
}

micronfig::config! {
	GARASAUTO: String => crate::MyCustomStruct,
}

fn main() {
	std::env::set_var("GARASAUTO", "me");
	assert_eq!(GARASAUTO(), &MyCustomStruct("me"));
}
