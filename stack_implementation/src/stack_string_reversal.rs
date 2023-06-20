// ------------------------------
// String Reversal with a Stack
// ------------------------------


fn main() {
    let input_string = String::from("I'm awesome!");
    let stack_size = input_string.len();
    let mut stack = new_stack(stack_size);

    let mut rev_string = String::new();

    for c in input_string.chars() {
        push(&mut stack, c, stack_size);
    }

    for i in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("Input string: {:?}", input_string);
    println!("Output string: {:?}", rev_string);
}

fn new_stack(max_size: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped_value: Option<char> = stack.pop();
    popped_value
}

fn push(stack: &mut Vec<char>, item: char, max_size: usize) {
    if stack.len() == max_size { return; }
    stack.push(item);
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn input() -> char {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: char = input.trim().parse().expect("Invalid input");
    input
}

