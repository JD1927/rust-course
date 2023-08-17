#[derive(Debug)]
struct Item {
    id: i32,
    title: String,
    year: i32,
    item_type: ItemType,
}
#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

fn display_item_info(item: Item) -> Item {
    let item_type = match item.item_type {
        ItemType::Book => String::from("Book"),
        ItemType::Magazine => String::from("Magazine"),
    };
    println!("Item");
    println!("ID: {}", item.id);
    println!("title: {}", item.title);
    println!("publication year: {}", item.year);
    println!("type: {}", item_type);
    item
}

fn main() {
    let my_book: Item = Item {
        id: 123,
        title: String::from("Can't hurt me"),
        year: 2010,
        item_type: ItemType::Book,
    };
    let my_magazine: Item = Item {
        id: 1123,
        title: String::from("Why do you care?"),
        year: 2011,
        item_type: ItemType::Magazine,
    };
    let returned_book = display_item_info(my_book);
    let returned_magazine = display_item_info(my_magazine);
    println!("{:?}", returned_book);
    println!("{:?}", returned_magazine);
}
