micronfig::config! {
	MY_IP_ADDR: String > IpAddr,
}

fn main() {
	println!("{:?}", MY_IP_ADDR())
}
