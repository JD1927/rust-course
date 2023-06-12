fn main() {
    // Vectors - Better to use than arrays
    // Resizable array

    let mut number_vec: Vec<i32> = vec![4, 5, 6, 12, 0, 33, 4];
    println!("{}", number_vec[0]);
    println!("{:?}", number_vec);
    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let array_with_same_elements: Vec<i32> = vec![0; 10];

    let mut string_array_1 = vec!["apple", "tomato", "grapes"];
    let string_array_2 = vec!["unknown"; 10];

    string_array_1[0] = "kiwi";

    let char_vec = vec!['1', '1', '0', 'a', 'a'];

    let subset_vec = &&number_vec[0..3];
    println!("Subset: {:?}", subset_vec);

    println!("Elements count: {:?}", number_vec.len());
    let check_index = number_vec.get(100);
    println!("number_vec[100]: {:?}", check_index);

    number_vec.push(30);
    number_vec.push(40);

    println!("number_vec: {:?}", number_vec);

    number_vec.remove(5);
    println!("number_vec: {:?}", number_vec);

    println!("Value of 10 exist: {}", number_vec.contains(&10));
}
