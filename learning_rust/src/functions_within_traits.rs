// ---------------------------------
// Functions within a Trait
// ---------------------------------
// Functions inside a trait can be used one to another.


struct Data {
    some_data: Vec<i32>,
}

trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}

impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.some_data.iter() {
            sum += *i;   
        }
        sum as f32 / self.some_data.len() as f32
    }

    fn variance(&self) -> f32 {
        let mean: f32 = self.mean();
        let mut squared_diff: f32 = 0.0;

        for i in self.some_data.iter() {
            squared_diff += (*i as f32 - mean) * (*i as f32 - mean);
        }

        squared_diff / self.some_data.len() as f32
    }
}

fn main() {
    let my_data: Data = Data {
        some_data: vec![5, 9, 8, 2, 3, 5, 9],
    };
    println!("Mean of the data is: {}", my_data.mean());
    println!("Variance of the data is: {}", my_data.variance());
}
