fn main() {
    // Tuple
    let my_info = ("Salary", 40_000);
    // Accessing values of the tuple by index
    println!("{} is {}", my_info.0, my_info.1);
    // Printing the whole Tuple
    println!("Another way of printing the whole tuple is {:?}", my_info);

    // Destructuring
    let (salary, salary_value) = my_info;
    let salary = my_info.0;
    let salary_value = my_info.1;

    // Nested tuples
    let nested_tuple = (4, 5.0, (3, 2), "Hello");
    let element = nested_tuple.2 .0; // --> 3

    let empty_tuple = ();

    // Arrays
    let mut number_array: [i32; 5] = [4, 5, 6, 8, 9];
    // Print value at 0 index
    println!("{}", number_array[0]);
    // Print the whole array
    println!("{:?}", number_array);

    number_array[4] = 5;

    // Array of zeros
    let array_with_same_elements = [0; 10];
    println!("{:?}", array_with_same_elements);

    let mut string_array_1 = ["apple", "tomato", "grapes"];
    // Array of "Unknown"
    let string_array_2 = ["Unknown"; 6];
    string_array_1[0] = "Kiwi";
    // Char array
    let char_array = ['a', 'p', 'p', 'l', 'e'];
    
    // Slice - Works like C with references as pointers
    let mut number_array_1: [i32; 5] = [4, 5, 6, 7, 8];

    let subset_array = &number_array_1[0..3]; // [4, 5, 6]
    
    println!("The subset of values: {:?}", subset_array);

    let subset_array = &number_array_1[0..=3]; // [4, 5, 6, 8]
    println!("The subset of values: {:?}", subset_array);

    println!("Elements in the array are {}", number_array_1.len());
    println!("Bytes {}", std::mem::size_of_val(&number_array_1));

    // number_array_1[10] = 5;

    let check_index = number_array_1.get(100);
    println!("{:?}", check_index);
}
