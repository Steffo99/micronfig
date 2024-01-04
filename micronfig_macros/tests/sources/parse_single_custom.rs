#[derive(std::fmt::Debug, Eq)]
struct MyCustomStruct(String);

impl std::str::FromStr for MyCustomStruct {
	type Err = ();
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self(s.to_owned()))
	}
}

micronfig::config! {
	GARASAUTO: String > crate::MyCustomStruct,
}

fn main() {
	std::env::set_var("GARASAUTO", "keke");
	assert_eq!(GARASAUTO(), &MyCustomStruct("keke"));
}
