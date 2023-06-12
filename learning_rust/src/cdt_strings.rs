// Compound Data Type - Strings

fn main() {
    // Strings
    // - Fixed length strings (&str)
    // - Growable strings (String)
    let some_string = "Fixed length string";
    let mut growable_string = String::from("This string will grow");
    println!("The string is \"{}\"", growable_string);

    // Push function
    growable_string.push('s');
    println!("The string is \"{}\"", growable_string);
    // Pop function
    growable_string.pop();
    println!("The string is \"{}\"", growable_string);

    growable_string.push_str(" which I will use");
    println!("The string is \"{}\"", growable_string);

    println!(
        "Basic Functions on Strings,
        is_empty(): {},
        length(): {},
        Bytes(): {},
        Contains use: {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use"),
    );

    growable_string.push_str("    ");
    let str_len = growable_string.trim().len();

    let number = 6;
    let num_str = number.to_string();

    println!("Is the number a string: {}", num_str == "6");

    let some_char = 'a';
    let char_str = some_char.to_string();

    let my_name = "JD1927".to_string();

    let empty_string = String::new();

    println!("Length is: {}", empty_string.len());

    let s_1 = "JD1927".to_string();
    let s_2 = "User".to_string();
    let s_3 = format!("{} is a Github's {}", s_1, s_2);
    println!("{}", s_3);

    let concat = format!("{}{}", s_1, s_2);
    println!("{}", concat);
}
