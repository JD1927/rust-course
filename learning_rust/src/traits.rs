// ---------------------------------
// Traits and Default Implementation
// ---------------------------------

struct Person {
    citizenship: String,
    name: String,
    age: u8,
    gender: char,
    salary: i32
}

struct Student {
    name_std: String,
    age: u8,
    sex: char,
    country: String,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char);
    fn country_info(&self) -> &str;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.country
    }
}

fn main() {
    let person1: Person = Person {
        citizenship: String::from("Colombian"),
        name: String::from("Juan Aguirre"),
        age: 24,
        gender: 'M',
        salary: 40_000,
    };
    let student1: Student = Student {
        name_std: String::from("David Aguirre"),
        age: 19,
        sex: 'M',
        country: String::from("Colombia")
    };

    println!("The basic info include {:?}", person1.info());
    println!("The basic info for the student is {:?}", student1.info());
}
