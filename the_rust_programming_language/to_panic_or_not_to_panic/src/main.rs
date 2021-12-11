fn main() {
}

// Create a type with values in a specific range
// to establish a contract.
// Functions taking Guess as an argument already know
// it's gonna be a value between 1 and 100.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
