// ----------------------------
// TUPLE STRUCTS BASICS
// ----------------------------

struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 >= self.1 {self.0} else {self.1}
    }
    fn lesser(&self) -> i32 {
        if self.0 < self.1 {self.0} else {self.1}
    }
}

fn main() {
    let some_numbers: Numbers = Numbers(33, 16);
    println!("The values of the two fields are {} and {}", some_numbers.0, some_numbers.1);
    println!("Greater of two numbers is {}", some_numbers.greater());
    println!("Lesser of two numbers is {}", some_numbers.lesser());
}
