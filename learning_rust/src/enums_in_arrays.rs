// ----------------------------------------
// Enums with vectors
// ----------------------------------------
#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.2)];
    println!(
        "The value of the integer is {:?} and the float is {:?}",
        some_val[0], some_val[1]
    );

    for i in some_val.iter() {
        match i {
            Value::Integer(num) => println!("The value of the integer is {}", num),
            Value::Float(num) => println!("The value of the float is {}", num),
        }
    }
}
