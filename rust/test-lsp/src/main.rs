fn main() {
    let x = 69;

    {
        let _y = 420;
        {
            let _z = 42;
        }
    }
    println!("Hello, {}!", x);
}
