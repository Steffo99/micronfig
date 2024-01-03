#[derive(std::fmt::Debug)]
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
	println!("{:?}", GARASAUTO())
}
