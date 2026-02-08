fn main() {
    println!("{}", fahrenheit_to_celsius(30.0))
}

fn fahrenheit_to_celsius(t: f64) -> f64 {
    (t - 32.0) * (5.0 / 9.0)
}
