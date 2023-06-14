fn main() {
  // Rust Ownership
  /*
      1. Each value in Rust has a variable that's called its owner.
      2. There can be only on owner at a time.
      3. When the owner goes out of scope, the value will be dropped.
   */

  // Floats

  let x = 32.6; // Primitive
  let y = x; // Copy of x - Fixed size

  println!("x: {}, y: {}", x, y);
  
  let s1 = String::from("abc"); // Non-primitive
  let s2 = s1; // Move of the value from s1 to s2 - Changes the ownership
  
  
  // println!("s1: {}, s2: {}", s1, s2); -- Causes an error
  // ----------------------------------
  // The way to fix it is by using the reference
  let s1 = String::from("abc");
  // Storing the reference of the variable s1 in s2 - Like in C
  // References do not change ownership
  let s2 = &s1; 
  
  
  println!("s1: {}, s2: {}", s1, s2);

  // Vectors

  let vec_1 = vec![5,6,7,8,9];
  let vec_2 = vec_1.clone();

  println!("Vec 1: {:?}, Vec 2: {:?}", vec_1, vec_2);

  // Scope

  {
      let my_name = String::from("Juan");
  }
  // Rust drop out of memory the variable my_name because of the scope
  println!("My name is {}", my_name);


}
