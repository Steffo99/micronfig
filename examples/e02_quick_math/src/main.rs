use std::fmt::{Display, Formatter};
use std::str::FromStr;


micronfig::required!(FIRST, u64);
micronfig::required!(SECOND, u64);
micronfig::required!(OPERATOR, Operator);


fn main() {
    let result = match *OPERATOR {
        Operator::Sum => (*FIRST) + (*SECOND),
        Operator::Subtraction => (*FIRST) - (*SECOND),
        Operator::Multiplication => (*FIRST) * (*SECOND),
        Operator::Division => (*FIRST) / (*SECOND),
    };

    println!("{} {} {} = {}", *FIRST, *OPERATOR, *SECOND, result)
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