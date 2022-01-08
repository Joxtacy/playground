trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    let person = Human;

    Pilot::fly(&person);
    <Human as Pilot>::fly(&person); // This also works
    Wizard::fly(&person);
    // Human::fly(&person); // This works the same as the row below
    person.fly();

    // This uses the function defined on the Dog struct
    println!("A baby dog is called a {}", Dog::baby_name());
    // This calls the function implemented for Dog as the Animal trait
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Fully Qualified Syntax
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);
}
