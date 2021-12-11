use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};
use std::error::Error;

// main function can have the following return type besides just ()
// this does require a return of Ok(()) in the main function though,
// but enables the use of the ? operator
fn main() -> Result<(), Box<dyn Error>> {
    // let f: u32 = File::open("hello.txt"); // kinda hacky way to make the compiler
                                             // tell us what type File::open returns
    let f = File::open("hello.txt");

    // let f = match f {
        // Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // A more elaborate version with handling of NotFound error
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Another version of the same thing
    let f = File::open("derp.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("derp.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap and expect automatically calls panic! in case of error.
    // expect lets us define an error message
    // let f = File::open("herp.txt").unwrap();
    // let f = File::open("herp.txt").expect("Failed to open herp.txt");

    let s = read_username_from_file().expect("Problem reading username from file");
    println!("Username read from file: {}", s);

    let f = File::open("henlo.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// using the ? operator to simplify implementation
fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut f = File::open("henlo.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

// using chaining to simplify further
fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("henlo.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// using std::io::read_to_string as this does exactly the same as above functions
fn read_username_from_file_v4() -> Result<String, io::Error> {
    fs::read_to_string("henlo.txt")
}
