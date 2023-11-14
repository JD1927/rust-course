// Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers. N will be a user-defined input that your program will take.

// For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.

// Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.

// Finally, calculate the difference as 225 - 55 = 170.

use std::io::stdin;

fn main() {
    // Get input from terminal
    let number: i32 = handle_input();

    println!("Number entered: {:?}", number);

    let calculations = perform_calculations(&number);

    println!("The square of the sum is: {:?}", calculations.0);
    println!("The sum of the square is: {:?}", calculations.1);
    println!("The difference is: {:?}", calculations.0 - calculations.1);

}

fn handle_input() -> i32 {
    // Get input from terminal
    let mut input = String::new();

    println!("Enter an integer number: ");
    stdin().read_line(&mut input).expect("Failed to read line");

    // Validate input 
    let number: i32 = input.trim().parse().expect("Invalid input");

    println!("Number entered: {:?}", number);
    number
}

fn perform_calculations(num: &i32) -> (i32, i32) {
    let mut square_of_the_sum: i32 = 0;
    let mut sum_of_the_squares: i32 = 0;

    let square = |num: i32| num * num;

    for i in 1..=*num {
        square_of_the_sum += i;
        sum_of_the_squares += square(i);
    }

    square_of_the_sum = square(square_of_the_sum);

    (square_of_the_sum, sum_of_the_squares)
}
