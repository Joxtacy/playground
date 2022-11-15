struct Mutable {
    x: i32,
    y: String,
}

fn main() {
    let mut mutable = Mutable {
        x: 69,
        y: String::from("Hello"),
    };

    mutable.x = 42;
    mutable.y.push_str(", ");

    let y = &mut mutable.y;
    y.push_str("mutable");

    println!("{}", mutable.y);
}
