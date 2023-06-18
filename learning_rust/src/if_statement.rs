/*
    Condition IF and its variations
 */

fn main() {
    let some_number = 55;
    if some_number < 50 {
        println!("Number less than 50");
    }
    println!("This is executed nevertheless");

    let marks = 65;
    if marks >= 60 && marks <=70 {
        println!("Grade is good enough");
    }

    let flag_1 = true;
    let flag_2 = false;

    if flag_1 == true || flag_2 == true {
        println!("The or operator works");
    }

    let flag_1 = true;
    if flag_1 != false  {
        println!("This is probably running")       
    }

    let flag_1 = true;
    let flag_2 = false;
    let number = 60;

    if (flag_1 && !flag_2) || number < 50 {
        println!("It will execute depending on the condition");
    }

    let marks = 80;
    
    if marks > 50 {
        println!("Passed!");
    } else {
        println!("Nope!");
    }
    
    
    let marks = 95;
    let mut grade: char = 'N';

    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("Student grade: {}", grade);

}
