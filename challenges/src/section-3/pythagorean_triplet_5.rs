// A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2. These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

// Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000

fn main() {
    let target_sum = 1000;

    for a in 1..(target_sum / 3) {
        let b_numerator = (1000 * a) - 500000;
        let b_denominator = a - 1000;

        if b_numerator % b_denominator == 0 {
            let b: i32 = b_numerator / b_denominator;
            let c = target_sum - a - b;

            println!("Pythagorean triplet: ({}, {}, {})", a, b, c);
            println!("Product of triplet: {}", a * b * c);
            return;
        }
    }
}
