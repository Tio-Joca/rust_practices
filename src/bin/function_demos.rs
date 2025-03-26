use rust_practices::*;

fn main () {
    const CELSIUS: f64 = 40.0;
    const FAHRENHEIT: f64 = 100.0;
    const KELVIN: f64 = 73.0;

    let mut result: f64;

    result = celsius_to_fahrenheit(CELSIUS);
    println!("{CELSIUS} ºC is equal to {result} ºF.");

    result = celsius_to_kelvin(CELSIUS);
    println!("{CELSIUS} ºC is equal to {result} ºK.");

    result = fahrenheit_to_celsius(FAHRENHEIT);
    println!("{FAHRENHEIT} ºF is equal to {result} ºC.");

    result = fahrenheit_to_kelvin(FAHRENHEIT);
    println!("{FAHRENHEIT} ºF is equal to {result} ºK.");

    result = kelvin_to_celsius(KELVIN);
    println!("{KELVIN} ºK is equal to {result} ºC.");

    result = kelvin_to_fahrenheit(KELVIN);
    println!("{KELVIN} ºC is equal to {result} ºF.");

    let mut number: i64;
    let mut verifier: bool;

    number = 4;

    verifier = is_even(number);
    println!("4 is even? {verifier}.");

    verifier = is_odd(number);
    println!("4 is odd? {verifier}.");

    number = 5;

    verifier = is_even(number);
    println!("5 is even? {verifier}.");

    verifier = is_odd(number);
    println!("5 is odd? {verifier}.");

    let gcd: i128 = gcd(1024, 478);
    println!("The greatest common divisor of 1024 and 478 is {gcd}.");

    let lcm: i128 = lcm(23, 45);
    println!("The least common multiple of 23 and 45 is {lcm}.");

    let bmi: f64 = body_mass_index(93.0, 1.75);
    println!("The body mass index of a person with 93.0 kg and 1.75 m is {bmi}.");

    let factorial: i128 = factorial(30);
    println!("The factorial of 30 is {factorial}.");
}
