use std::thread;
use std::hash::Hash;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Eq + Hash + Copy,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U: Eq + Hash + Copy> Cacher<T, U>
where 
    T: Fn(U) -> U,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
