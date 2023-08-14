// ----------------------------------------
// Hash Maps
// ----------------------------------------
use std::collections::HashMap;

fn main() {
    // let mut person: HashMap<&str, i32> = HashMap::new();
    // person.insert("Juan", 40);
    // person.insert("David", 24);
    // person.insert("Juliana", 22);

    // println!("The age is {:?}", person.get("Juan").unwrap());

    // if person.contains_key("Juan") {
    //     println!("Value exist!");
    // } else {
    //     println!("Value does not exist!");
    // }

    // match person.get("Juan") {
    //     Some(value) => println!("The value exist {}", value),
    //     None => println!("The value does not exist"),
    // };

    // for (name, age) in &person {
    //     println!("Person {} has an age of {}", name, age);
    // }

    // let mut likes: HashMap<&str, &str> = HashMap::new();

    // likes.insert("Juan", "Grapes");
    // likes.insert("Juan", "Watermelon");
    // println!("What fruit do I like? : {:?}", likes);

    // likes.entry("Juan").or_insert("Watermelon");
    // likes.entry("Juan").or_insert("Grapes");
    // println!("What fruit do I like? : {:?}", likes);

    let some_vec = vec![4,4,2,3,8,1,1,3,9,8];
    let mut freq_vec: HashMap<i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }

    println!("{:?}", freq_vec);

}
