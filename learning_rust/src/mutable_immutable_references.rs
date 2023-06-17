/*
    Reference Rules
        - One mutable reference in a scope
        - Many immutable references
        - Mutable and immutable cannot coexist
        - Scope of a reference
        - Data should not change when immutable references are in scope
 */

fn main() {
    // Rule 1 -------------------------------
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &mut heap_num;
    // It ensures there are not data races.
    let ref2 = &mut heap_num; // --> Causes an error because Rust does not allow to concurrently update the same reference value at the same time
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // Rule 2 -------------------------------
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num; // --> This is correct
    let ref2 = &heap_num; // --> This is allowed as long as is immutable
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    // Rule 3 -------------------------------
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    let ref3 = &mut heap_num; // --> Causes an error because mutable and immutable references cannot coexist
    println!("ref1: {:?}, ref2: {:?}, ref3: {:?}", ref1, ref2, ref3);
    
    // Rule 4 -------------------------------
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2); // It is safe because until this point the data is already presented, which means the scope is over for ref1 and ref2
    let ref3 = &mut heap_num;
    
    // Rule 5 -------------------------------
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    
    heap_num.push(68); 
    // --> Causes an error because Rust does not allow the changing of the data value of a variable while there are immutable readers for the data
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);
    

}
