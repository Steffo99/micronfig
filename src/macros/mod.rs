//! Highest-level API — Define lazy statics.
//!
//! The recommended way to use the library.


/// Define a required configuration value with a certain type.
///
/// # Process
///
/// This macro:
///
/// 1. uses [`lazy_static::lazy_static`] to define a new static variable (and associated struct)
/// 2. uses [`crate::handle::get_required`] to get the configuration value and handle eventual errors
///
/// # Examples
///
/// Define a configuration value with the `USER` key, a [`String`]:
/// ```
/// # std::env::set_var("USER", "steffo");
/// # std::env::remove_var("USER_FILE");
/// #
/// micronfig::required!(USER, String);
/// println!("{:?}", *USER);
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS` environment variable or the `IP_ADDRESS_FILE` file, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// #
/// # std::env::set_var("IP_ADDRESS", "192.168.1.1");
/// # std::env::remove_var("IP_ADDRESS_FILE");
///
/// micronfig::required!(IP_ADDRESS, IpAddr);
/// println!("{:?}", *IP_ADDRESS);
/// ```
#[macro_export]
macro_rules! required {
    ($identifier:ident, $kind:ty) => {
        $crate::lazy_static::lazy_static! {
            pub(crate) static ref $identifier: $kind = $crate::handle::get_required::<$kind>(stringify!($identifier));
        }
    };
}

/// Define a optional configuration value with a certain type.
///
/// # Process
///
/// This macro:
///
/// 1. uses [`lazy_static::lazy_static`] to define a new static variable (and associated struct)
/// 2. uses [`crate::handle::get_optional`] to get the configuration value and handle eventual errors
///
/// # Examples
///
/// Define a configuration value with the `USER` key, a [`String`]:
/// ```
/// # std::env::set_var("USER", "steffo");
/// # std::env::remove_var("USER_FILE");
/// #
/// micronfig::optional!(USER, String);
/// println!("{:?}", *USER);
/// ```
///
/// Retrieve a configuration value from the `IP_ADDRESS` environment variable or the `IP_ADDRESS_FILE` file, then try to convert it to a [`std::net::IpAddr`]:
/// ```
/// use std::net::IpAddr;
/// #
/// # std::env::set_var("IP_ADDRESS", "192.168.1.1");
/// # std::env::remove_var("IP_ADDRESS_FILE");
///
/// micronfig::optional!(IP_ADDRESS, IpAddr);
/// println!("{:?}", *IP_ADDRESS);
/// ```
#[macro_export]
macro_rules! optional {
    ($identifier:ident, $kind:ty) => {
        $crate::lazy_static::lazy_static! {
            pub(crate) static ref $identifier: Option<$kind> = $crate::handle::get_optional::<$kind>(stringify!($identifier));
        }
    }
}
