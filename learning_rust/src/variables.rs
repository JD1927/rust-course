fn main() {
    let x: f32 = 15.0;
    let mut y: f32 = 15.0;
    println!("The value of variable x = {}", x);
    // x = 60; --> Causes an error, variables are not mutable by default
    y = 60; // --> It is correct!

    // Rules for variables naming conventions: Letters, digits and underscores
    let X = 10; // Different than "x" variable
    // ----------------------------------------------

    // ----------------------------------------------
    // SCALAR TYPE
    // Primitive types - Scalar types
    // integers, floats, booleans and characters
    // ----------------------------------------------
    // Integers:
    // - Signed: i8, i16, i32, i64
    // - Unsigned: u8, u16, u32, u64
    println!("The maximum number in i8 = {}", std::i8::MAX);
    println!("The maximum number in i16 = {}", std::i16::MAX);

    // ----------------------------------------------
    // Floats:
    // - f32
    // - f64

    let z: f64 = 3.65;
    let a = X + z; // Causes an error because it cannot implicitly convert data types

    // ----------------------------------------------
    // Boolean:

    let status = false;
    println!(
        "The value of some of out variables are {:?}",
        (x, y, z, status)
    );

    let not_equals: bool = 18 != 18;
    println!("The value of condition is = {}", not_equals);

    // ----------------------------------------------
    // Characters:

    let c1 = 'a';
    let c2 = '3';
    let c4 = '+';

}
