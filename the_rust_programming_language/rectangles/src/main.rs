fn main() {
    // version 1
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // version 2
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // version 3
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    // println!("rect2 is {}", rect2); // ❌ Display trait not implemented
    println!("rect2 is {:?}", rect2); // ✅ Debug trait added
    println!("rect2 is {:#?}", rect2); // ✅ Debug trait added
    dbg!(&rect2); // writes to stderr, needs Debug trait
    
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect4.area());

    if rect4.width() {
        println!("The rectangle has a non zero width; it is {}", rect4.width);
    }


    // Can Hold
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("This is a square with an area of {} pixels", sq.area());
}

// version 1
fn area1(width: u32, height: u32) -> u32 {
    width * height
}

// version 2
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//version 3
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)] // version 3
struct Rectangle {
    width: u32,
    height: u32,
}

// functions with the &self param are called on an instance
// functions without &self can be called without an instance 
impl Rectangle {
    // can be &mut self if we want to mutate, or just self if we want ownership (rare)
    fn area(&self) -> u32 { // &self is short for self: &Self, where &Self is an alias for Rectangle
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    // The &Self will mean it accepts &Rectangle since we are in a impl Rectangle block
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
