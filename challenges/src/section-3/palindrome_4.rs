
// A palindrome is a word, verse, or sentence that reads the same backward or forward, such as 'Able was I ere I saw Elba,' or a number like 1881.

// Write a function named is_palindrome() that checks whether a given string is a palindrome or not. The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.

use std::io::stdin;


fn main() {
    let input = handle_input();
    let is_palindrome = is_palindrome(&input);
    println!("{:?}", is_palindrome);
}
fn handle_input() -> String {
    // Get input from terminal
    let mut input = String::new();

    println!("===========[Palindrome checker]=============");
    println!("Enter a value: ");
    stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn is_palindrome(value: &str) -> bool {
    let copy_value = value.trim().to_lowercase().split_whitespace().collect::<String>();
    let reverse_value = copy_value.chars().rev().collect::<String>();
    copy_value == reverse_value
}
