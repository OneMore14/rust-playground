#![allow(unused)]

fn main() {

    fn double_first(vec: Vec<&str>) -> i32 {
        let first = vec.first().unwrap(); // Generate error 1
        2 * first.parse::<i32>().unwrap() // Generate error 2
    }

    let numbers = vec!["42", "93", "18"];
    //let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    //println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    //println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();

    }
}