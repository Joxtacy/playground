use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // Borrowing twice is not allowed. But we borrow from different parts of the slice,
    // so it is okay, but Rust doesn't understand that.
    // (&mut slice[..mid], &mut slice[mid..])

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Calling external code. This block creates a Foreign Function Interface (FFI).
// "C" defines which application binary interface (ABI) the function uses.
extern "C" {
    fn abs(input: i32) -> i32;
}

// This defines a function to be called from C.
// #[no_mangle] means that the compiler doesn't mangle the function name.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    let number: i32 = -3;
    unsafe {
        println!("Absolute value of {} according to C: {}", number, abs(number));
    }

    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
