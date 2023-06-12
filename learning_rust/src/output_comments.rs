fn main() {
  // This is our first program in this course
  // This is the second line of comment

  /*
      This is a multi-line comment
      Hell yeah!
  */
  // println!() - Works like the println in C programming language

  println!("Hello from Rust!");
  // println!(10); --> Causes error, it expects a string literial
  println!("The value is {}", 10);

  println!(
      "My first name is {} and my last name is {}",
      "Juan", "Aguirre"
  );
  // print!() - Works like the println in C programming language
  print!("This will be printed in the same line");
  print!("This will be printed in the same line");

  // With escape sequences
  println!("\n\nThis will be printed in another line");

  println!("\t This will be printed in another line");

  println!("This will be printed in \r another line");

  println!("This will print single quote \' and this double quotes \"");

  println!(
      "I'm doing {2} from {1} years and I {0} it",
      "like", "20", "programming"
  );

  println!(
      "{language} is system programming language which is cool to {activity} in.",
      language = "Rust",
      activity = "code"
  );

  println!("The result of 25 + 10 = {}", 25 + 10);

}
