// -----------------------
// Function Types - Functional pointers
// -----------------------

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn prints_name(name: &str) {
    println!("The name is {}", name);
}
fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
    f(some_one);
    println!(" and my age is {}", age);
}

fn add_one(x: i32) -> i32 {
    x+1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}



fn main() {
    // let mut f = max;
    // println!("The maximum of the two values is {}", f(2,3));
    // println!("The minimum of the two values is {}", f(2,3));

    // let (my_name, my_age) = (String::from("Juan"), 40);
    // prints_full_info(prints_name, &my_name, my_age);

    let answer = do_twice(add_one, 5);
    println!("{}", answer);

}
