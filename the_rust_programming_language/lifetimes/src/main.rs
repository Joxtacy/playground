// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

// Generic Type Parameters, Trait Bounds, and Lifetimes
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // String literals always have 'static lifetime
    let s: &'static str = "I have a static lifetime";

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");

        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}

// missing lifetime specifier.
// introduce lifetime specifiers
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest1<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// lifetimes in functions must be in relation to parameters
fn longest_dangling_reference<'a>(x: &str, _y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
