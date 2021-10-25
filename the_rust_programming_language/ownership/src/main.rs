fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s);

    // Ownership example
    let s1 = String::from("hello");
    let s2 = s1; // this invalidates s1

    // println!("{}, world!", s1); // can't use s1 since it has been moved to s2
    println!("{}, world!", s2);

    // cloning
    let s1 = String::from("hello");
    let s2 = s1.clone(); // possibly very expensive operation. AVOID

    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and functions
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // this does not work since s lost its ownership

    let x = 69;

    makes_copy(x);

    println!("{}", x); // this still works because x was copied, not moved

    // more examples
    let s1 = gives_ownership(); // s1 get ownership from String created in takes_ownership

    let s2 = String::from("hello"); // s2 get ownership of new String

    let s3 = takes_and_gives_back(s2); // s2 loses ownership to takes_and_gives_back
                                       // s3 gets ownership from takes_and_gives_back

    let s1 = String::from("henlo");

    let (s2, len) = calculate_length(s1); // s1 loses ownership. a tuple was returned and here dereferenced

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
