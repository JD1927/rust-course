use std::collections::HashMap;

struct Student {
    name: String,
    age: i32,
    grade: String,
}

fn add_student(
    student_database: &mut HashMap<i32, Student>,
    id: i32,
    name: String,
    age: i32,
    grade: String,
) {
    let new_student: Student = Student { name, age, grade };
    student_database.entry(id).or_insert(new_student);
}

fn main() {
    let mut student_database: HashMap<i32, Student> = HashMap::new();

    add_student(
        &mut student_database,
        1,
        String::from("Juan"),
        24,
        String::from("Grade 1"),
    );
    add_student(
        &mut student_database,
        2,
        String::from("David"),
        24,
        String::from("Grade 0"),
    );

    for (id, student) in &student_database {
        println!("Student ID: {}", id);
        println!("Name: {}", student.name);
        println!("Age: {}", student.age);
        println!("Grade: {}", student.grade);
        println!("--------------------------");
    }
}
