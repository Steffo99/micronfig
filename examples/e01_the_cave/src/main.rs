use std::fmt::{Display, Formatter};
use std::str::FromStr;


micronfig::required!(ECHO, String);


fn main() {
    println!("{}", *ECHO);
}
