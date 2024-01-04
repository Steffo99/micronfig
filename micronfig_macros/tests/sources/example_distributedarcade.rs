use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr};

micronfig::config! {
	REDIS_CONN: String,
	AXUM_HOST: String > std::net::SocketAddr,
	CREATE_TOKEN: String,
}

fn main() {
	std::env::set_var("REDIS_CONN", "redis://garas");
	std::env::set_var("AXUM_HOST", "127.0.0.1:12345");
	std::env::set_var("CREATE_TOKEN", "tokennnnn");

	assert_eq!(REDIS_CONN(), "redis://garas");
	assert_eq!(AXUM_HOST(), &SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 12345)), "127.0.0.1:12345");
	assert_eq!(CREATE_TOKEN(), "tokennnnn");
}
