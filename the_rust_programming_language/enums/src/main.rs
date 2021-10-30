fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    #[derive(Debug)]
    struct IpAddrDumb {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddrDumb {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrDumb {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    dbg!(&home);
    dbg!(&loopback);

    // More concise version of the above
    // Enums can have different variants. structs are all the same
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    // Another enum example
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
            dbg!(&self);
            match &self {
                Self::Write(s) => println!("This is Message::Write(\"{}\")", s),
                Self::Quit => println!("herp"),
                Self::ChangeColor(a, b, c) => println!("derp: {} {} {}", a, b, c),
                _ => {
                    println!("all other cases");
                    ()
                }
            }
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrKind) {}
