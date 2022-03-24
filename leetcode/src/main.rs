use std::cmp::min;
use std::collections::HashMap;

struct Solution{

}
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {

        let mut number1 = vec![0; 500];
        let mut number2 = vec![0; 500];
        let mut v1 : Vec<_>= version1.split('.').collect();
        let mut v2 : Vec<_>= version2.split('.').collect();
        for i in 0..v1.len() {
            number1[i] = v1[i].parse::<i32>().unwrap();
        }
        for i in 0..v2.len() {
            number2[i] = v2[i].parse::<i32>().unwrap();
        }
        let ans = 0;
        for i in 0..500 as usize {
            if number1[i] > number2[i] {
                return 1;
            } else if number1[i] < number2[i] {
                return -1;
            }
        }
        ans
    }
}


fn main () {


    println!("hello world")
}