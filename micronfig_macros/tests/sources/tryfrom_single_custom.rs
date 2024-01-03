#[derive(std::fmt::Debug)]
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
	println!("{:?}", GARASAUTO())
}
