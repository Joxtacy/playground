use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    let derp = 43.0 % 12.3; // what does this even do?
    println!("herp derp {}", derp);
    let derp = 43.0 / 12.3;
    println!("herp derp {}", derp);

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}, and the value of z is: {}", y, tup.2);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [2; 5]; // creates [2, 2, 2, 2, 2]

    let first = a[0];
    let second = a[1];

    // invalid array element access example
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
