// ----------------------------------------
// Result Enums
// ----------------------------------------

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
//     if divisor == 0.0 {
//         Err(String::from("Error: Division by zero is not possible."))
//     } else {
//         Ok(dividend / divisor)
//     }
// }

fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    match divisor {
        0.0 => Err(String::from("Error: Division by zero is not possible.")),
        _ => Ok(dividend / divisor),
    }
}

fn main() {
    println!("{:?}", division(9.0, 3.0));
    println!("{:?}", division(9.0, 0.0));
    println!("{:?}", division(3.0, 1.0));

    let some_vec = vec![5, 5, 6, 9, 7, 1];
    let result1 = match some_vec.get(15) {
        Some(a) => Ok(a),
        None => Err("Value does not exist"),
    };
    println!("The value of the result is {:?}", result1);
}
