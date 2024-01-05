use std::env;

#[derive(Debug, PartialEq, Eq)]
struct GuildId(u64);

#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

impl From<u64> for GuildId {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl From<u64> for UserId {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

micronfig::config! {
	ANGY_TOKEN: String,
	ANGY_APPID: String > u64,
	ANGY_PLEX_SERVER: String,
	ANGY_PLEX_TOKEN: String,
	ANGY_PLEX_LIBRARY: String,
	ANGY_PLEX_REPLACE_FROM: String,
	ANGY_PLEX_REPLACE_TO: String,
	ANGY_DEV_GUILD_ID?: String > u64 -> crate::GuildId,
	ANGY_DEV_USER_ID?: String > u64 -> crate::UserId,
}

fn main() {
	env::set_var("ANGY_TOKEN", "abcdef");
	env::set_var("ANGY_APPID", "1234");
	env::set_var("ANGY_PLEX_SERVER", "example.org");
	env::set_var("ANGY_PLEX_TOKEN", "123456dada");
	env::set_var("ANGY_PLEX_LIBRARY", "beta");
	env::set_var("ANGY_PLEX_REPLACE_FROM", "sus");
	env::set_var("ANGY_PLEX_REPLACE_TO", "sos");
	env::set_var("ANGY_DEV_GUILD_ID", "4567");
	env::set_var("ANGY_DEV_USER_ID", "5678");

	assert_eq!(ANGY_TOKEN(), "abcdef");
	assert_eq!(ANGY_APPID(), &1234);
	assert_eq!(ANGY_PLEX_SERVER(), "example.org");
	assert_eq!(ANGY_PLEX_TOKEN(), "123456dada");
	assert_eq!(ANGY_PLEX_LIBRARY(), "beta");
	assert_eq!(ANGY_PLEX_REPLACE_FROM(), "sus");
	assert_eq!(ANGY_PLEX_REPLACE_TO(), "sos");
	assert_eq!(ANGY_DEV_GUILD_ID(), &Some(GuildId(4567)));
	assert_eq!(ANGY_DEV_USER_ID(), &Some(UserId(5678)));
}
