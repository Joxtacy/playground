#![feature(yeet_expr)]

fn divide_option(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        do yeet
    }
    Some(x / y)
}

#[derive(Debug)]
enum DivErr {
    DivideByZero,
    Overflow,
}

fn divide_result(x: i32, y: i32) -> Result<i32, DivErr> {
    if y == 0 {
        do yeet DivErr::DivideByZero
    }

    match x.checked_div(y) {
        Some(r) => Ok(r),
        None => do yeet DivErr::Overflow,
    }
}
fn main() {
    println!("=== OPTION ===");
    let res = divide_option(3, 0);
    println!("{:?}", res);
    let res = divide_option(42, 2);
    println!("{:?}", res);
    println!("=== OPTION ===");
    println!("=== RESULT ===");
    let res = divide_result(3, 0);
    println!("{:?}", res);
    let res = divide_result(69, 4);
    println!("{:?}", res);
    println!("=== RESULT ===");
}
