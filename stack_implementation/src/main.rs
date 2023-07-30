/*  ------------------------------
    Expression Evaluation infix to postfix
    ------------------------------

    -> Convert to postfix
    -> Evaluate expression from postfix

    Operands: Constant values, a,b,c
    Operators: *+-/^

    (a + b) = a b +
      infix = postfix

    * Infix to Postfix
        Rules:
            1. Priorities of operators:
                a. + -
                b. * /
                c. ^
            2. If scanned operator is <> The top operator of the stack in priority then we pop the operators until we have low priority.
            3. If "(" -> push it to the stack
            4. If ")" -> pop elements until "("
            5. If operand then just add it to postfix expression

    Example: (33 + 45 / 3 * (2 + 9) - 50)
            Symbol      Stack       Postfix
    1.       (             (
    2.      33             (            33
    3.      +              (+           33
    4.      45             (+           33, 45
    5.      /              (+/          33, 45
    6.      3              (+/          33, 45, 3
    7.      *              (+*          33, 45, 3, /
    8.      (              (+*(         33, 45, 3, /
    9.      2              (+*(         33, 45, 3, /, 2
    10.     +              (+*(+        33, 45, 3, /, 2
    11.     9              (+*(+        33, 45, 3, /, 2, 9
    12.     )              (+*          33, 45, 3, /, 2, 9, +
    13.     -              (-           33, 45, 3, /, 2, 9, +, *, +
    14.     50             (-           33, 45, 3, /, 2, 9, +, *, +, 50
    15.     )                           33, 45, 3, /, 2, 9, +, *, +, 50, -


*/

/* 
    1. Priorities of operators
        -> +, -
        -> *, /
        -> ^
    2. If scanned operator is <= then the top of the stack in priority then pop operators until we have low priority. Add the popped elements to the postfix.

    3. If "(" push it to the stack

    4. If ")" pop elements until "(" and add popped elements to postfix

    5. If operand then just add to the postfix
*/



fn main() {
    // let input_expr = String::from("(33+45/3*(2+9)-50)");
    let input_expr = String::from("(44+33) * (34+39)-36");
    println!("The original expression is {:?}", input_expr);

    let input_expr_tokenized = get_expression_symbols(input_expr);
}

fn get_expression_symbols(input_expr: String) -> Vec<String> {
    let mut tokenized_input: Vec<String> = Vec::new();

    // Converts the result to any collection, such as vector of strings
    let input_chars: Vec<char> = input_expr.chars().collect();

    let mut temp: Vec<char> = Vec::new();

    for i in input_chars {
        if i != '+' && i != '-' && i != '*' && i != '/' && i != '^' && i != '(' && i != ')' {
            temp.push(i);
            continue;
        } else {
            if temp.len() == 0 {
                tokenized_input.push(i.to_string());
            } else {
                tokenized_input.push(temp.into_iter().collect());
                tokenized_input.push(i.to_string());
                temp = vec![];
            }
        }
    }

    if temp.len() != 0 {
        tokenized_input.push(temp.into_iter().collect());
    }
    println!("{:?}", tokenized_input);
    tokenized_input
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

fn infix_to_postfix(input: Vec<String>) -> Vec<String> {
    let size_expr = input.len();
    let mut stack = new_stack(size_expr);
    let mut postfix: Vec<String> = Vec::new();
    // TODO: Continue with video 124 Evaluation - Part 2 lesson.

    // for i in input  {
    //     match i.as_str() {
    //         "+" | "-" | "*" | "/" | "^" => {
    //             if priority(&i) > priority(stack.last().unwrap()) {
    //                 push(&mut stack, i, size_expr)
    //             } else {
    //                 while priority(&i) <= priority(stack.last().unwrap()) {
    //                     postfix.push(pop(&mut stack).unwrap());
    //                     if stack.last() == None {
    //                         break;
    //                     }
    //                 }
    //                 push(&mut stack, i, size_expr);
    //             }
    //         }
    //         "(" => {}
    //         ")" => {}
    //         _ => {}
    //     }
    // }

    postfix
}

fn priority(input: &String) -> u8 {
    if ("+" == input) | ("-" == input) {
        1
    } else if ("*" == input) | ("/" == input) {
        2
    } else if "^" == input {
        3
    } else {
        0
    }
}