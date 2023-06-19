/*
    Match Statement
 */

fn main() {
    // match value {
    //     possible_value => { something to execute } 
    //     _ => { default execution }
    // }

    let some_number = 100;
    match some_number {
        1 => println!("The number is 1"),
        2 | 3 => println!("The number is 2 or 3"),
        4..=100 => println!("The number is between 4 and 100"),
        _ => println!("The number is greater than 100"),
    }

    let marks: i32 = 80;
    let mut grade: char = 'N';

    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F'
    }

    println!("The grade achieved is {}", grade);

    // Let match
    let marks: i32 = 95;
    let grade: char = match marks { // Returns the same type, char
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F'
    };

    println!("The grade achieved is {}", grade);


}
