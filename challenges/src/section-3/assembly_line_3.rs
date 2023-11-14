// This question involves writing code to analyze the production of an assembly line in a car factory. The assembly line has different speeds, ranging from 0 (off) to 10 (maximum). At the lowest speed of 1, the assembly line produces a total of 221 cars per hour. The production rate increases linearly with the speed, meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

// However, higher speeds increase the likelihood of producing faulty cars that need to be discarded. The success rate depends on the speed, as shown in the table below:

// · Speeds 1 to 4: 100% success rate.

// · Speeds 5 to 8: 90% success rate.

// · Speeds 9 and 10: 77% success rate.

// You need to write two functions:

// 1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.

// 2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.

// Write the code for both functions based on the provided specifications.

const CARS_PRODUCED_PER_HOUR_LINE: i32 = 221;

fn main() {
    let hours = 2;
    let speed = 1;
    let cars_produced = total_production(hours, speed);
    let cars_produced_per_minute = cars_produced_per_minute(hours, speed);
    println!("Cars produced in {:?} hours: {:?}", hours, cars_produced);
    println!("Cars produced per minute in {:?} hours: {:?}", hours, cars_produced_per_minute);
}

fn total_production(hours: i32, speed: i32) -> i32 {
    let mut cars_produced: i32 = 0;
    let success_rate: f32 = match speed {
        0 => 0.0,
        1..=4 => 1.0,
        5..=8 => 0.8,
        9.. => 0.77,
        _ => 0.77,
    };

    for _i in 1..=hours {
        let assembly_line = speed * CARS_PRODUCED_PER_HOUR_LINE;
        let result = assembly_line as f32 * success_rate;
        cars_produced += result.round() as i32;
    }
    cars_produced
}

fn cars_produced_per_minute(hours: i32, speed: i32) -> i32 {
    // Calculate cars produced per hour using the existing logic
    let cars_produced_per_hour = total_production(hours, speed);

    // Convert to cars produced per minute
    let cars_produced_per_minute = (cars_produced_per_hour as f32 / 60.0).round() as i32;

    cars_produced_per_minute
}
