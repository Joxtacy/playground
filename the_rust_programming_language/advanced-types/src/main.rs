
fn main() {

    // type alias
    // Kilometers is a synonym for i32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Creating type alias for code repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));
    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {}
    // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {}

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("hi"))
    }
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> { // --snip-- }

    // Types aliases can have generics as well
    type Result<T> = std::result::Result<T, std::io::Error>;
}
