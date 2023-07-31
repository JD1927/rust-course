// ----------------------------
// STRUCTS BASICS
// ----------------------------

struct Person {
    citizenship: String,
    name: String,
    age: i32,
    gender: char,
    salary: i32,
}

impl Person {
    fn new() -> Person {
        Self {
            name: String::from("Name"),
            citizenship: String::from("Citizenship"),
            age: 1,
            gender: 'X',
            salary: 1,
        }
    }

    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.0) * 0.5
    }
}

fn main() {
    // Declaration
    let person1: Person = Person {
        name: String::from("Juan Aguirre"),
        citizenship: String::from("Colombian"),
        age: 24,
        gender: 'M',
        salary: 124_000,
    };

    // Access
    println!(
        "The structure values are {} {} {}",
        person1.citizenship, person1.age, person1.gender
    );

    println!(
        "The taxes on Person {} are {}",
        person1.name,
        person1.compute_taxes()
    );
    // Same as the above
    println!(
        "The taxes on Person {} are {}",
        person1.name,
        Person::compute_taxes(&person1)
    );

    let person2 = Person::new();
    println!(
        "Default values for person 2 Name: {} Citizenship: {}",
        person2.name, person2.citizenship
    );

    let person3: Person = Person {
        age: 50,
        name: String::from("Karman"),
        ..person1
    };

    println!(
        "Person 3 is: {}, his salary is: {}",
        person3.name, person3.salary
    );

    let mut person4 = Person::new();

    println!("Person 4 is: {}", person4.name);
    person4.name = String::from("Levy");
    println!("Person 4 updated is: {}", person4.name);
}
