use std::fmt::{Display, Formatter};
use std::str::FromStr;


fn main() {
    let echo: String = micronfig::required("ECHO");

    println!("ECHOing back: {echo}");
}
