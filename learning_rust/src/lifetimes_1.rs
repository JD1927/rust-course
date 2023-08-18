//-------------------------------------
// Lifetimes
//-------------------------------------
/*
   Explains the scope for which a reference is being valid

   Note: Memory safe

   Why?
   Dangling reference is a reference which point to a resource whose owner does not exist.
   Access an invalid address
*/
// Borrow checker
// fn some_fn(i: &i32) -> &i32 {
//     &i
// }

// Undetermined lifetime case
// fn greater(i: &i32, j: &i32) -> &i32 {
//     if i > j {
//         i
//     } else {
//         j
//     }
// }

fn some_fn(first_str: &str, second_str: &str) -> &str {
    first_str
}

fn main() {
    // Example 1
    // let i: &i32;
    // {
    //     let j = 5;
    //     i = &j;
    //     // j dies in this scope so its not valid to reference it out of it
    // }
    // println!("i = {}", i);

    // Example 2
    // let some_int = 10;
    // let additional_int = some_fn(&some_int);
    // println!("{}", additional_int);

    // Example 3 - Undetermined lifetime
    // let i1 = 5;
    // let i2 = 10;

    // Example 4 - Undetermined lifetime
    let s_1: &str = "Hello";
    let v;
    {
        let s_2 = String::from("World!");
        v = some_fn(s_1, s_2.as_str());
    }
    println!("{}", v);

}
