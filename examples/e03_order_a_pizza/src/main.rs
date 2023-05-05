use std::fmt::Formatter;
use std::net::IpAddr;
use std::str::FromStr;


// The name of the person who ordered the pizza.
micronfig::required!(FULLNAME, String);

// The (IP) address the pizza should be delivered to.
micronfig::required!(DESTINATION, IpAddr);

// The base of the pizza to add toppings on.
micronfig::required!(PIZZABASE, PizzaBase);

// The toppings to add to the pizza.
micronfig::optional!(PIZZATOPPINGS, PizzaToppingsList);
// A pizza with no toppings, to use as fallback.
const PIZZATOPPINGS_NONE: PizzaToppingsList = PizzaToppingsList{ list: vec![] };


fn main() {
    // Let's print the order!
    println!("Pizza Order");
    println!("===========");
    println!();
    println!("Base:");
    println!("- {}", *PIZZABASE);
    println!();
    println!("Toppings:");
    for topping in &(*PIZZATOPPINGS).as_ref().unwrap_or(&PIZZATOPPINGS_NONE).list {
        println!("- {}", &topping);
    };
    println!();
    println!("Deliver to:");
    println!("{} @ {}", *FULLNAME, *DESTINATION)
}


/// A possible base of pizza.
#[derive(Clone, Copy, Debug)]
enum PizzaBase {
    /// Just the pizza dough, with nothing else on top f it.
    Blank,
    /// Pizza dough with tomato on top.
    Red,
    /// Pizza dough with mozzarella on top.
    White,
    /// Pizza dough with both tomato and mozzarella on top.
    Margherita,
}

impl FromStr for PizzaBase {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // Italian
            "vuota" => Ok(Self::Blank),
            "stria" => Ok(Self::Blank),
            "rossa" => Ok(Self::Red),
            "marinara" => Ok(Self::Red),
            "pomodoro" => Ok(Self::Red),
            "bianca" => Ok(Self::White),
            "mozzarella" => Ok(Self::White),
            "regina" => Ok(Self::Margherita),
            "margherita" => Ok(Self::Margherita),
            "normale" => Ok(Self::Margherita),
            "entrambi" => Ok(Self::Margherita),
            // English
            "blank" => Ok(Self::Blank),
            "red" => Ok(Self::Red),
            "tomato" => Ok(Self::Red),
            "white" => Ok(Self::White),
            "cheese" => Ok(Self::White),
            "both" => Ok(Self::Margherita),
            "normal" => Ok(Self::Margherita),
            // Unknown
            _ => Err("Unknown pizza base; ensure you have written the name in either English or Italian!"),
        }
    }
}

impl std::fmt::Display for PizzaBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PizzaBase::Blank => "Blank (Empty)",
            PizzaBase::Red => "Red (Tomato)",
            PizzaBase::White => "White (Mozzarella)",
            PizzaBase::Margherita => "Margherita (Tomato + Mozzarella)"
        })
    }
}

/// The toppings
#[derive(Clone, Debug)]
struct PizzaToppingsList {
    pub list: Vec<String>
}

impl FromStr for PizzaToppingsList {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<String> = s.split(",").map(|s| s.to_string()).collect();

        for topping in list.iter() {
            // Ensure compatibility with https://github.com/rust-lang/rust/pull/70645
            if ["pineapple", "ananas"].contains(&topping.as_str()) {
                return Err("Ruining pizzas is not allowed by the Rust compiler.")
            }
        }

        Ok(
            PizzaToppingsList {
                list
            }
        )
    }
}