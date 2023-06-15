/*
    [MEMORY SEGMENTS]
    * HEAP: 
        - Stores non-primitive values than are not in order
        - Putting data request a certain amount of space
        - Return the pointer/address of that location
        - Free large pool of memory
    * STACK: 
        - Stores the program instructions and code
    * STATIC / GLOBAL: 
        - Stores static and global variables 
        - Are accessible anywhere during the whole lifecycle of the program as long is executing
        - Visible to all parts of the program and are declared before the start of the main function
        - Typically constants
    * CODE - TEXT:
        - Stores all the information regarding the function calls 
        - Stores the local variables inside these functions and in the context of Rust, all the primitive type variables
        
    [NOTES]: 
        - The segments of Stack, Static and Code do not grow while the app is running
 */
/*
    [EXECUTING]

        1. The main() function is invoked, some amount of memory from the stack is allocated for execution of main
            - As MAX_VALUE is constant variable defined in a global scope then is added to the Static/Global Segment
        2. The amount of memory allocated in stack for execution of main function can also be called the stack frame for the main function.
        3. Stack frame
            -----------
                square()
                num
            -----------
                square_sum()
                num1, num2
            -----------
                main()
                x, y
            -----------
        
        The functions that are not at the top are paused/waiting condition
    
    When the program starts, the OS allocates some amount of reserved space. The actual allocation of the stack frames and the allocation of the local variables happen from the stack during runtime. 
    
    Stack overflow: If the stack grows beyond the reserved memory for the stack

    Example: Declaring variables inside an infinite loop

    All data stored on the stack must have known fixed size. 
    Data with an unknown size at compile time must be stored on the HEAP segment then.


 */

const MAX_VALUE: i32 = 40_000;
fn main() {
    let (x, y) = (2, 4);
    let sum_value = square_sum(x, y);
    println!("The value of Square of Sum = {}", sum_value);
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    let result = square(num1+num2);
    return result;
}

fn square(num: i32) -> i32 {
    num * num
}