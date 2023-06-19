/*
    - Break - Stopping a loop
    - Continue - Skip current iteration goes to the next one
 */

fn main() {
    let mut var = 100;
    loop {
        var -= 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!("The hightest number divisible by 13 is {}", var);

    let mut var = 0;
    let mut count = 0;
    loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("{} is divisible by 3 and 5", var);
            count += 1;
            if count == 3 {
                break;
            } else {
                continue;
            }
        }
        println!("{}", var);
    }

    let mut var = 0;
    let mut count = 0;
    let req_number = loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("{} is divisible by 3 and 5", var);
            count += 1;
            if count == 3 {
                // This only can be possible with the loop statement, not for or while loops
                break var; 
            } else {
                continue;
            }
        }
        println!("{}", var);
    };
    println!("Required number {}", req_number);
}
