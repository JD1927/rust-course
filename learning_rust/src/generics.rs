// ----------------------------------------
// Generics
// ----------------------------------------
// Later types

fn square<T>(x: T) -> T where T: std::ops::Mul<Output = T> + Copy {
    x*x
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T,U> where T: std::fmt::Debug,U: std::fmt::Debug {
    fn printing(&self) {
        println!("The value of the point coordinates are {:?}, {:?}", self.x, self.y);
    }
}


fn main() {
    let p1 = Point { x: 5, y: 5 };
    let p2 = Point { x: 1.0, y: 4.0 };
    let p3 = Point { x: 5, y: 5.0 };

    p1.printing();
    p2.printing();
    p3.printing();
}
