// --------------------------------
// Iterators - Sequences of values
// --------------------------------

fn main() {
    let some_vec = vec![0,1,2,3,4,5,6,7];

    // Creates an iterator inside a closure to determine whether the values should be yield or not
    let filtered_values = some_vec.iter().filter(|&x| *x >= 5).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);

    let a = some_vec.clone();
    let filtered_values = some_vec.into_iter().filter(|&x| x >= 5).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);
    
    // some_vec is not longer useful using the function into_iter - into_iter takes the values without a reference
    // println!("{:?}", some_vec); // -> Error
    
    // Map - Takes a closure and creates an iterator which calls that closure on each element and will return a new iterator
    let mapped_values = a.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

    let mapped_values = a.iter().map(|x| 2 * *x).filter(|x| *x > 10).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);



}
