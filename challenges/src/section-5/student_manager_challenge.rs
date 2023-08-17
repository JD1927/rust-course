use std::collections::HashMap;
#[derive(Debug)]
struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(String::from("Student's ID already exists!"))
        } else {
            self.students.entry(student.id).or_insert(student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

fn main() {
    let mut student_manager: StudentManager = StudentManager::new();
    let student1: Student = Student {
        id: 1,
        name: String::from("Juan"),
        grade: String::from("Grade 1"),
    };
    let student2: Student = Student {
        id: 2,
        name: String::from("David"),
        grade: String::from("Grade 1"),
    };
    student_manager.add_student(student1).unwrap();
    student_manager.add_student(student2).unwrap();

    println!("{:?}", student_manager.get_student(1));
    println!("{:?}", student_manager.get_student(2));
}
