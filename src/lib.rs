use std::fmt::Debug;

pub mod sources;
pub mod cache;

#[macro_export]
macro_rules! __micronfig_last {
	[ $head:ty, $( $tail:ty, )+ ] => {
		$crate::__micronfig_last![ $( $tail, )+ ]
	};
	[ $head:ty, ] => {
		$head
	}
}

/// # Examples
///
/// ## Get a string directly
///
/// ```
/// micronfig::config! {
/// 	MY_STRING_A: String,
/// 	MY_STRING_B: String
/// }
/// ```
///
/// ## Parse envvar as a number
///
/// ```
/// micronfig::config! {
/// 	MY_UNSIGNED_NUMBER: u32,
/// 	MY_SIGNED_NUMBER: i32
/// }
/// ```
///
/// ## Parse string as an IpAddr
///
/// ```
/// use std::net::IpAddr;
///
/// micronfig::config! {
/// 	MY_IP_ADDR: IpAddr
/// }
/// ```
///
/// ## Parse string as a number, then convert it into an Ipv4Addr
///
/// ```
/// use std::net::Ipv4Addr;
///
/// micronfig::config! {
/// 	MY_NUMERIC_IP_ADDR: Ipv4Addr
/// }
/// ```
///
/// ## Parse string with custom logic resulting into a number
///
/// ```
/// struct CustomConverter(u8);
///
/// impl From<String> for CustomConverter {
/// 	fn from(value: String) -> Self {
///         Self(123)
///     }
/// }
///
/// impl Into<u8> for CustomConverter {
/// 	fn into(self) -> u8 {
///         self.0
///     }
/// }
///
/// micronfig::config! {
/// 	MY_ONETWOTHREE: CustomConverter => u8,
/// 	MY_ONETWOTHREE_BUT_EXPLICIT: String => CustomConverter => u8
/// }
/// ```
///
#[macro_export]
macro_rules! config {
	{ $( $identifier:ident: $( $conversion:ty )=>* ),+ } => {
		static __micronfig_cache: std::sync::OnceLock<$crate::cache::MicronfigCache> = std::sync::OnceLock::new();

		fn __micronfig_init_cache() -> $crate::cache::MicronfigCache {
			let mut this = $crate::cache::MicronfigCache::default();

			if cfg!(feature = "envdot") {
				this.add_envdot("./.env");
				this.add_envdot("./.env.local");
			}

			this
		}

		fn __micronfig_get(key: &str) -> Option<String> {
			let mut value: Option<String> = None;

			if cfg!(feature = "envfiles") && value.is_none() {
				value = $crate::sources::envfiles::get(format!("{key}_FILE"));
			}

			if cfg!(feature = "envvars") && value.is_none() {
				value = $crate::sources::envvars::get(&key);
			}

			if cfg!(feature = "envdot") && value.is_none() {
				let cache = __micronfig_cache.get_or_init(__micronfig_init_cache);

				for dotenv in cache.dotenvs.iter() {
					value = $crate::sources::envdot::get(dotenv, &key);
					if value.is_some() {
						break;
					}
				}
			}

			value
		}

		$(
			pub(self) mod $identifier {
				pub static lock: std::sync::OnceLock<Option< $crate::__micronfig_last![ $( $conversion, )+ ] >> = std::sync::OnceLock::new();
			}

			pub(crate) fn $identifier () -> &'static Option< $crate::__micronfig_last![ $( $conversion, )+ ] > {
				$identifier::lock.get_or_init(|| {
					let key = stringify!($identifier);
					let value = __micronfig_get(key);

					$(
						let value: Option<$conversion> = value.map(Into::into);
					)+

					value
				})
			}
		)+
	}
}

config! {
	SOMETHING: String
}
