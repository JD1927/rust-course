/*
    - Loops with no condition
    - While loop
 */
use std::io::{self, stdin};

fn main() {
    // loop {
    //     println!("This is an infinite loop");
    // }

    // let my_number = 5;
    // let mut guess = false;

    // println!("Guess my number which is between 1 and 20");

    // while !guess {
    //     let mut number = String::new();
    //     std::io::stdin()
    //         .read_line(&mut number)
    //         .expect("Failed to read input");

    //     let number: u8 = number.trim().parse().expect("Invalid input");

    //     if my_number == number {
    //         println!("You guessed the number!");
    //         guess = true;
    //     } else {
    //         println!("Try again!!!");
    //     }
    // }

    println!("Enter a number and I'll tell you the next number after your number that is divisible by 2 and 5");

    let mut number = String::new();
    stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    let mut number: u8 = number.trim().parse().expect("Invalid input");
    
    number += 1;
    while !(number % 2 == 0 && number % 5 == 0) {
        number += 1;
    }
    println!("The number {} is divisible by 2 and 5", number);

}
