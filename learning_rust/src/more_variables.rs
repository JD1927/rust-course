fn main() {
  let (first_number, second_number) = (250, 480.22);
  let large_number = 1_000_000;

  // Integer Overflow
  // let over_flow_number: u8 = 256; --> Causes an error because is out of range
  let x = 255;

  println!(
      "The value of the variable in octal is {:o} and in hexadecimal is {:X} and binary {:b}",
      x, x, x
  );

  let number = 45; // snake_case

  let n1 = 14;
  let n2 = 15.6;
  // Convert types
  // let n3 = n1 + n2 as i32;
  let n3 = n1 as f64 + n2;
  println!("The value of n3 = {}", n3);

  // SHADOWING: Quote segments use case
  let s = 5;
  let s = 5 * 5;

  println!("The value of s is {}", s);

  let mut p = 5;
  let p = 5 * 5;

  // p = 60;

  let q = 32;
  let q = 'A';

  // Quote segments

  let mut r = 65;
  {
      let r = 60;
      println!("Inside the code segment r: {}", r);
  }
  println!("Outside the code segment r: {}", r);

  // CONSTANTS - Add type, Uppercase

  const MAX_SALARY: u32 = 100_000;
}
