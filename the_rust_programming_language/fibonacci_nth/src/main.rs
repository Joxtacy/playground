fn main() {
    println!("1st Fibonacci number: {}", fib(1));
    println!("6th Fibonacci number: {}", fib(6));
    println!("13th Fibonacci number: {}", fib(13));
    println!("69th Fibonacci number: {}", fib(69));
}

fn fib(n: u64) -> u64 {

    let mut prev = 0;
    let mut curr = 1;

    if n == 1 {
        return 0;
    }

    let mut count = 2;
    while count < n {
        let temp = curr;
        curr += prev;
        prev = temp;

        count += 1;
    }

    curr
}
