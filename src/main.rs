mod lib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num1 = &args[1];
    let num2 = &args[2];

    let sum = lib::add(num1.parse::<usize>().unwrap(), num2.parse::<usize>().unwrap());
    println!("Sum of 2 numbers is {}", sum);
}