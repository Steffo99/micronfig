use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() {
    let echo: String = micronfig::get("ECHO")
        .expect("ECHO configuration value to be defined");

    println!("ECHOing back: {echo}");
}
