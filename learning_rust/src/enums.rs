// ----------------------------------------
// Enums
// ----------------------------------------

enum Conveyance {
    Car(i32),
    Train(i32),
    Air(i32),
    Walk,
}
impl Conveyance {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            Conveyance::Car(miles) => *miles as f32 * 14.0 * 2.0,
            Conveyance::Train(miles) => *miles as f32 * 18.0 * 2.0,
            Conveyance::Air(miles) => *miles as f32 * 30.0 * 2.0,
            _ => 0.0
        };
        allowance
    }
}

fn main() {
    let participant_1:Conveyance = Conveyance::Car(60);
    let participant_2:Conveyance = Conveyance::Train(120);
    let participant_3:Conveyance = Conveyance::Air(60);
    println!("The participant 1 has a travel allowance of {}", participant_1.travel_allowance());
    println!("The participant 2 has a travel allowance of {}", participant_2.travel_allowance());
    println!("The participant 1 has a travel allowance of {}", participant_3.travel_allowance());
}
