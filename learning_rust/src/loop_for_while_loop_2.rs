/*
    - Loops with no condition
    - While loop
 */
use std::io::{self, stdin};

fn main() {
    loop {
        println!("This is an infinite loop");
        break; // -> This breaks the loop
    }

    // To be out the loop it is also possible to use labeling.
    'outer: loop {
        println!("This is an infinite loop");
        break 'outer; // -> This breaks the specific loop
    }

    // Assigns the break value to the variable
    let a = loop {
        break 5;// --> Maybe for a failing operation
    };

    let vector = vec![45, 30, 85, 90, 71, 39];

    for i in vector {
        println!("{i}");
    }

    let mut num = 0;

    while num < 10 {
        num = num + 1;
    }


}
