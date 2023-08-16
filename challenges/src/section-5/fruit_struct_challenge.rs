struct Fruit {
    apples: i32,
    bananas: i32,
}

fn increase_fruit(fruit: Fruit) -> Fruit {
    Fruit {
        apples: fruit.apples * 2,
        bananas: fruit.bananas * 2,
    }
}

fn new_fruit() -> Fruit {
    Fruit {
        apples: 10,
        bananas: 5,
    }
}

fn println_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

fn main() {
    let some_fruit: Fruit = new_fruit();
    let updated_fruit: Fruit = increase_fruit(some_fruit);
    println_fruit(updated_fruit);
}
