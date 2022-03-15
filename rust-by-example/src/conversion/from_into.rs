#![allow(dead_code)]
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: u32,
}

impl From<u32> for Number {
    fn from(item: u32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[cfg(test)]
mod tests {
    use crate::conversion::from_into::main;

    #[test]
    fn test_main() {
        main()
    }
}