fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisable by 4");
    } else if number % 3 == 0 {
        println!("number is divisable by 3");
    } else if number % 2 == 0 {
        println!("number is divisable by 2");
    } else {
        println!("number is not divisable by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "six" }; // can't use mismatching expression types

    println!("The value of number is: {}", number);
}
