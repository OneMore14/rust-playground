use std::fmt;

struct Circle {
    radius: i32
}

// fmt::Display automagically provides std::string::ToString
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();  // trait FromStr
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

#[cfg(test)]
mod tests {
    use crate::conversion::to_and_from_strings::main;

    #[test]
    fn test_main() {
        main()
    }
}