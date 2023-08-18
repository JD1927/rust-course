//-------------------------------------
// Lifetimes P2
//-------------------------------------


struct Person<'a> {
    name: &'a str,
    age: i32,
}

fn main() {
    // Example 1
    // let s_1: &str = "Hello";
    // let v;
    // {
    //     let s_2 = String::from("World!");
    //     v = some_fn(s_1, s_2.as_str());
    // }
    // println!("{}", v);

    // Example 2
    // let int1 = 5;
    // let int2 = 10;
    // let result = greater(&int1, &int2);

    // Example 3
    // let int1 = 5;
    // let int2 = 10;
    // let result = greater(&int1, int2);


    // Example 4
    // let int1 = 5;
    // {
    //     let int2 = 10;
    //     let result = greater(&int1, &int2);
    //     println!("{}", result);
    // }
    // println!("{}", result); -> Error

    // Example 5
    // Both structure and field have the same lifetime
    // let first_name = "Juan";
    // let mut juan: Person = Person {
    //     name: &first_name,
    //     age: 24
    // };
    // {
    //     let last_name = String::from("Aguirre");
    //     juan.name = &last_name;
    //     println!("Person name {} and age {}", juan.name, juan.age);
    // }
    // println!("Person name {} and age {}", juan.name, juan.age);

    // Example 6
    let some_vec = vec![5,5,6,6,2,4,7,6,22,3];
    let return_vec = user_vec(&some_vec, &some_vec);
}

// fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
//     first_str
// }

// fn greater(i: &i32, j: &i32) -> i32 {
//     if i > j {
//         *i
//     } else {
//         *j
//     }
// }

// fn greater<'a>(i: &'a i32, j: i32) -> &'a i32 {
//     i
// }

// fn greater<'a, 'b>(i: &'a i32, j: &'a i32) -> &'a i32 {
//     if i > j {
//         i
//     } else {
//         j
//     }
// }

fn user_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
}