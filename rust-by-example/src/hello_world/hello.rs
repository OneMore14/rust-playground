
pub fn hello() {
    println!("Hello World!");
    println!("I'm a Rustacean!")
}

#[cfg(test)]
mod tests {
    use crate::hello_world::hello::hello;

    #[test]
    fn test_hello() {
        hello();
    }
}