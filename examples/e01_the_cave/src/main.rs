use std::fmt::Display;


micronfig::required!(ECHO, String);


fn main() {
    println!("{}", *ECHO);
}
