use std::fmt::{Display, Formatter};
use std::str::FromStr;


fn main() {
    let first: u64 = micronfig::required("FIRST");
    let second: u64 = micronfig::required("SECOND");
    let operator: Operator = micronfig::required("OPERATOR");

    let result = match operator {
        Operator::Sum => first + second,
        Operator::Subtraction => first - second,
        Operator::Multiplication => first * second,
        Operator::Division => first / second,
    };

    println!("{first} {operator} {second} = {result}")
}


pub enum Operator {
    Sum,
    Subtraction,
    Multiplication,
    Division,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Sum),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),
            _ => Err(())
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Sum => "+",
            Self::Subtraction => "-",
            Self::Multiplication => "*",
            Self::Division => "/",
        })
    }
}