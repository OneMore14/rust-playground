use std::char::from_digit;
use crate::basic_data_structure::stack::Stack;

fn divide_by_two(mut dec_num: u32) -> String {

    if dec_num == 0 {
        return "0".to_string();
    }
    let mut s = Stack::new();
    while dec_num > 0 {
        s.push(dec_num % 2);
        dec_num /= 2;
    }
    let mut str = "".to_string();
    while !s.is_empty() {
        str.push(from_digit(*s.peek().unwrap(), 10).unwrap());
        s.pop();
    }

    return str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {

        assert_eq!("0", divide_by_two(0));
        assert_eq!("1", divide_by_two(1));
        assert_eq!("110", divide_by_two(6));
    }
}