use linked_list::List;

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    list = list.prepend(7);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
