/*
    Rust Ownership in Functions

    - Each value in Rust has a variables that's called its owner
    - There can be only one owner at a time
    - When the owner goes out of scope, the value will be dropped
 */

 fn main() {
  let stack_num = 32;
  let mut heap_vec = vec![4, 5, 6];
  stack_function(stack_num);
  println!("Value inside the main of stack_num: {}", stack_num);

  heap_function(&mut heap_vec);
  println!("Value inside the main of heap_vec: {:?}", heap_vec);

  let some_vec = vec![4, 5, 6];
  let ref1 = some_vec; // --> Owner
  let ref2 = &ref1;

  let mut vec_1 = vec![4, 5, 6];
  let mut ref1 = &mut vec_1;

  let large_data1 = String::from("This is the first long string");
  let large_data2 = String::from("This is the second long string");

  // Physical - Copies
  // Conceptual - References not copies
  let huge_data = vec![&large_data1, &large_data2];
}
/*
  The variable stack_num is just a copy of the variable stack_num declared in the main function.
  It does not affect the execution
  - Owner did not change
*/
fn stack_function(mut stack_num: i32) {
  stack_num = 56;
  println!("Var: {}", stack_num);
}

fn stack_function2(mut stack_num: i32) -> i32 {
  stack_num = 56;
  println!("Var: {}", stack_num);
  return stack_num;
}
/*
  When a variable is on the heap and is passed to a function, its value is moved to that of the function
  - Move occurs
  - Owner changed
*/
fn heap_function(var: &mut Vec<i32>) {
  var.push(50);
  println!("Var: {:?}", var);
}

