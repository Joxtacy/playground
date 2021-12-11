fn main() {
    let b = Box::new(5); // stores the i32 on the heap instead of the stack
    println!("b = {}", b);
}
