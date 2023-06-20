// ------------------------------
// STACK IMPLEMENTATION
// ------------------------------


fn main() {
    println!("[Stack Implementation]");
    println!("Enter the size of the stack: ");
    let stack_size: usize = input() as usize;
    
    let mut stack: Vec<u32> = new_stack(stack_size);
    
    loop {
        println!("\n---------MENU--------");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Display");
        println!("4. Size");
        println!("5. Exit");
        println!("---------------------");
        println!("\nEnter your choice: ");
    
        let choice: u32 = input();
    
        match choice {
            1 => {
                println!("Enter the value to insert: ");
                let item: u32 = input();
                push(&mut stack, item, stack_size);
            },
            2 => println!("Popped value: {:?}", pop(&mut stack)),
            3 => println!("Stack: {:?}", stack),
            4 => println!("Stack size: {:?}", size(&stack)),
            5 => {
                println!("\n Bye!");
                break;
            },
            _ => {
                println!("\nInvalid option. Try again");
                continue;
            },
        }
        println!("\nDo you want to continue? [Yes] = 1 / [No] = 0");
        let status: u32 = input();
        if status == 1 {
            continue;
        } else {
            break;
        }
    }

}

fn new_stack(max_size: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(max_size);
    vec
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    let popped_value: Option<u32> = stack.pop();
    println!("Popped value is {:?}", popped_value);
    popped_value
}

fn push(stack: &mut Vec<u32>, item: u32, max_size: usize) {
    if stack.len() == max_size {
        println!("Cannot add more items to the stack");
        return;
    }
    stack.push(item);
    println!("Stack: {:?}", stack);
}

fn size(stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: u32 = input.trim().parse().expect("Invalid input");
    input
}

