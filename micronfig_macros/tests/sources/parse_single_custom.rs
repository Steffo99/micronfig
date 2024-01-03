#[derive(std::fmt::Debug)]
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
	println!("{:?}", GARASAUTO())
}
