fn main() {
    let temp = 100.0;

    println!("{} degrees C is {} degrees F", temp, c_to_f(temp));
    println!("{} degrees F is {} degrees C", temp, f_to_c(temp));
}

fn c_to_f(temp: f64) -> f64 {
    temp * (9.0 / 5.0) + 32.0
}

fn f_to_c(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}
