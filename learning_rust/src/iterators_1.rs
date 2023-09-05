// --------------------------------
// Iterators - Sequences of values - For
// --------------------------------

// .iter() fucntion
fn main() {
    let some_vec = vec![0,2,3,4,5,6,7];
    // No effect until it's consumed
    let mut iter = some_vec.iter();

    // All of the values of the vector get capture into the iter variable
    println!("The iterator {:?}", iter);
    
    println!("{:?}", iter.next()); // Some(0)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(3)
    println!("{:?}", iter.next()); // Some(4)
    println!("{:?}", iter.next()); // Some(5)
    println!("{:?}", iter.next()); // Some(6)
    println!("{:?}", iter.next()); // Some(7)
    println!("{:?}", iter.next()); // None

    // Any function like some in JS
    let mut check = some_vec.iter().any(|&x| x > 0);
    println!("The value of the any function is {}", check);

    // All function like every in JS
    let mut check = some_vec.iter().all(|&x| x >= 0);
    println!("The value of the all function is {}", check);

    // Find the first element that matches the predicate
    let check = some_vec.iter().find(|&&x| x > 0);
    println!("The value of the find function is {}", check.unwrap());

    // Position of the element
    let check = some_vec.iter().position(|&x| x > 4);
    println!("The value of the position function is {}", check.unwrap());

    // RPosition of the element
    let check = some_vec.iter().rposition(|&x| x > 4);
    println!("The value of the rposition function is {}", check.unwrap());
    
    // Max value of the list
    let check = some_vec.iter().max();
    println!("The value of the max function is {}", check.unwrap());

    // Min value of the list
    let check = some_vec.iter().min();
    println!("The value of the min function is {}", check.unwrap());

    // Reverse values of the list
    let mut iter = some_vec.iter().rev();
    println!("Reverse values of the list {:?}", iter);
    println!("{:?}", iter.next());

    
}
