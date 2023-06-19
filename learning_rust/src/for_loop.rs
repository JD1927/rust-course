/*
   - For loop and its variations
*/

fn main() {
    let mut some_vec = vec![10, 21, 34, 45, 59, 69];
    for i in 0..=5 {
        println!("{} : {}", i, some_vec[i]);
    }
    // The values of the vector are being consumed. So there is not possible to print the vector after this. Ownership changed
    // for value in some_vec {
    //     println!("{}", value);
    // }

    // --> Error, the values of some_vec has been borrowed
    // println!("{:?}", some_vec);

    // IMMUTABLE REFERENCES
    // The values of the vector are not being consumed with .iter()
    for value in some_vec.iter() {
        println!("{}", value);
    }

    // --> Correct
    println!("{:?}", some_vec);
    // MUTABLE REFERENCES

    // The values of the vector are not being consumed with .iter_mut()
    for value in some_vec.iter_mut() {
        *value += 5; // The '*' character is for accessing and modifying the pointer
        println!("{}", value);
    }

    // --> some_vec values + 5 to the original array
    println!("{:?}", some_vec);

    // It works like the iter_mut() function with mutable references
    for value in &mut some_vec {
        *value -= 5; // The '*' character is for accessing and modifying the pointer
        println!("{}", value);
    }

    // --> some_vec values + 5 to the original array
    println!("{:?}", some_vec);
}
