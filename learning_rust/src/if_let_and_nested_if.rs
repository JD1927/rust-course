/*
   Nested If, If let (in case of if else ladder)
*/

fn main() {
    println!("Hello, world!");

    println!("Enter a number");
    let mut some_num: String = String::new();

    std::io::stdin()
        .read_line(&mut some_num)
        .expect("Failed to read input");

    let some_num: i32 = some_num.trim().parse().expect("Invalid input");

    if some_num != 0 {
        if some_num % 2 == 0 {
            println!("Even");
        } else {
            println!("Odd");
        }
    } else {
        println!("The number is 0 :s");
    }

    // If let
    let value = if true { 1 } else { 2 };

    println!("Value = {}", value);

    let marks = 95;
    let grade: char = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else {
        'F'
    };

    println!("Student grade: {}", grade);
}
