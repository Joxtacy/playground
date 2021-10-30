fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("joxtacy"),
        active: true,
        sign_in_count: 1,
    };

    println!("This is the email: {}", user1.email);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("joxtacy"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another_email@example.com");

    println!("This is the email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // struct update syntax. Like javascript spread operator, but must come last
    }; // can no longer use user1 since the username field was moved

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(a, b, c) = black; // desctructure a named tuple
    println!("{}, {}, {}", a, b, c);
    println!("{}", origin.1);

    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn build_user(email: String, username:  String) -> User {
    User {
        email,    // same as email: email
        username, // same as username: username
        active: true,
        sign_in_count: 1,
    }
}
