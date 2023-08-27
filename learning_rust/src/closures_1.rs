//-----------------------------------
// Closures
//-----------------------------------
// |...| {...} - anonymous functions
// Reusable
// Captures its environment - all inside the body

fn main() {
    // Example 1
    // let x = 5;
    // let square = |num: i32| println!("The square of the variable is {}", num*num);
    // // Change the ownership
    // let square = |num: i32| println!("The square of the variable is {}", num * num * num);
    // square(x);

    // let y = 15;
    // square(y);

    // Example 2
    // let print_info = |general_info: String, name: &str, age| println!("{}\n\t {}: {}", general_info, name, age);
    
    // let general_info = String::from("The details are ");
    // let (person_name, person_age) = (String::from("Juan"), 24);
    // print_info(general_info, &person_name, person_age);
    
    // println!("The variable has been moved {}", person_age);


    // Example 3
    // Closures are able to infer the outputs and inputs themselves. Unlike functions where it is required to explicitly set up the types

    // let square = |num| num * num;
    // let x = 5;
    // square(x);
    
    // let y = 1.5;
    // square(y);

    let division_status = |y: f32| {if y != 0.0 {true} else {false}};

    division(5.0, 10.0, division_status);
    division(5.0, 0.0, division_status);

}


fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
    if f(y) == true {
        println!("The division result is {}", x/y);
    } else {
        println!("The division is not possible");
    }
}