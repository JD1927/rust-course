//-----------------------------------
// Closures
//-----------------------------------
// |...| {...} - anonymous functions

fn main() {
    // let some_closure_1 = |x: u32| -> u32 { x + 1 };
    // let some_closure_2 = |x| { x + 1 };
    // let some_closure_3 = |x| x + 1;

    // let mut vec_1 = vec![1,2,3];
    // let mut some_closure = || {
    //     vec_1.push(10);
    // };
    // some_closure();
    // println!("{:?}", vec_1);
    
    let mut vec_1 = vec![1,2,3];
    let mut some_closure = || {
        let vec2 = vec_1; // Lost the ownership
    };
    some_closure();
    println!("{:?}", vec_1);
    println!("{:?}", vec_2);


}
