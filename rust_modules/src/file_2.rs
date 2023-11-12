mod person {
    // Public structure. Fields are private by default in a structure. Even if the struct is public
    pub struct personal_info {
        pub age: u8,
        pub education: String,
    }
    impl personal_info {
        pub fn new(new_edu: &str) -> Self {
            Self {
                age: 24,
                education: String::from(new_edu),
            }
        }
    }
}

pub fn some_person() {
    let mut person1 = person::personal_info::new("Bachelors");
    person1.education = String::from("Masters");

    let mut person2 = person::personal_info {
        age: 24,
        education: String::from("Masters"),
    };
}
