use std::str::FromStr;

#[derive(Debug)]
pub struct CommaSeparatedStrings(Vec<String>);

impl FromStr for CommaSeparatedStrings {
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self(s.split(',').collect())
	}
}

micronfig::config! {
	DATA_DRAGON_LOCALE: String,
	DATA_DRAGON_SET_CODES: String > crate::CommaSeparatedStrings,
	POROXY_KEY: String,
	POROXY_SALT: String,
	SERENITY_DEV_GUILD_ID?: String > u64,
}

fn main() {
	std::env::set_var("DATA_DRAGON_LOCALE", "it_IT");
	std::env::set_var("DATA_DRAGON_SET_CODES", "set1,set2abc");
	std::env::set_var("POROXY_KEY", "abcdef");
	std::env::set_var("POROXY_SALT", "abcdef");
	std::env::remove_var("SERENITY_DEV_GUILD_ID");

	assert_eq!(DATA_DRAGON_LOCALE(), "it_IT");
	assert_eq!(DATA_DRAGON_SET_CODES(), CommaSeparatedStrings(vec!["set1", "set2abc"]));
	assert_eq!(POROXY_KEY, "abcdef");
	assert_eq!(POROXY_SALT, "abcdef");
	assert_eq!(SERENITY_DEV_GUILD_ID, &None);
}
