fn main() {
    basic_fn();
    function_with_inputs("JD", 40_000);
    let full_name = "Juan";
    let salary_info = 50_000;

    function_with_inputs(full_name, salary_info);

    let answer = function_with_input_and_outputs(10, 2);
    println!("10 * 2 = {}", answer);


    let (multiply, addition, subtraction) = function_with_input_and_multiple_outputs(10, 2);
    println!(
        "Multiplication = {}, Addition = {}, Subtraction = {}",
        multiply, addition, subtraction
    );

    let result = function_with_input_and_multiple_outputs(10, 2);
    println!(
        "Multiplication = {}, Addition = {}, Subtraction = {}",
        result.0, result.1, result.2
    );

    // Code blocks

    let full_name = {
        let first_name = "Juan";
        let last_name = "Aguirre";

        format!("{} {}", first_name, last_name)
    };

    println!("{}", full_name);

    let mut n: String = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("Something went wrong reading the input");
    // Convert to types

    let n: f64 = n.trim().parse().expect("Invalid input");

    println!("{:?}", n);

}

fn basic_fn() {
    println!("This is a basic function");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("Name {} and salary {}", name, salary);
}

fn function_with_input_and_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_input_and_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}