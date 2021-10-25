fn main() {
    let s1 = String::from("henlo");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // this example does not work. can't borrow immutable
    // let s = String::from("henlo");
    // change(&s);

    // this example does work with all the mut in it
    let mut s = String::from("henlo");
    change(&mut s);

    // new example
    let mut s = String::from("henlo");

    let r1 = &mut s;
    // let r2 = &mut s; // can't borrow s more than once

    // println!("{}, {}", r1, r2);


    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM. Users of an immutable reference don't expect the value to change.

    // println!("{}, {}, and {}", r1, r2, r3);


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem because r1 and r2 are not used after this point
    println!("{}", r3);


    let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// fn change(some_string: &String) {
    // some_string.push_str(", wurld!");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", wurld!");
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
  // We could instead return the String as it is and instead transfer ownership
