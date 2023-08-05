// ---------------------------------
// Traits and Default Implementation
// ---------------------------------

struct Circle {
    radius: f32
}

struct Rectangle {
    length: f32,
    width: f32,
}
struct Square {
    width: f32,
}

trait GeneralInfo {
    fn area(&self) {
        println!("I'm not implemented yet!")
    }
    fn perimeter(&self);
}

impl GeneralInfo for Circle {
    // fn area(&self) {
    //     let area: f32 = 3.14 * (self.radius * self.radius);
    //     println!("Area of the circle is: {}", area);
    // }
    fn perimeter(&self) {
        let circumference: f32 = 2.0 * 3.14 * self.radius;
        println!("Circumference of the circle is: {}", circumference);
    }
}
impl GeneralInfo for Rectangle {
    fn area(&self) {
        let area: f32 = self.length * self.width;
        println!("Area of the rectangle is: {}", area);
    }
    fn perimeter(&self) {
        let perimeter: f32 = (self.length * 2.0) + (self.width * 2.0);
        println!("Perimeter of the rectangle is: {}", perimeter);
    }
}

impl GeneralInfo for Square {
    fn area(&self) {
        let area: f32 = self.width * self.width;
        println!("Area of the square is: {}", area);
    }
    fn perimeter(&self) {
        let perimeter: f32 = (self.width * 4.0);
        println!("Perimeter of the square is: {}", perimeter);
    }
}



fn main() {
    let c1: Circle = Circle {
        radius: 3.2
    };
    let r1: Rectangle = Rectangle {
        length: 5.0,
        width: 4.0,
    };
    let s1: Square = Square {
        width: 4.0,
    };

    // Circle
    c1.area();
    c1.perimeter();

    // Rectangle
    r1.area();
    r1.perimeter();

    // Square
    s1.area();
    s1.perimeter();
}
