fn main () {
    // Immutable variable
    let x: i8 = 127;
    println!("The value of x is {x}.");

    // Mutable variable
    let mut y: i16 = 1023;
    println!("\nThe value of y is {y}.");
    y = 2048;
    println!("\nThe value of y is {y}.");

    // Constant
    const A: i32 = 1200123430;
    println!("\nThe value of A is {A}.");

    // Shadowing
    let b: i64 = 17;

    let b: i64 = b * b;

    {
        let b: i64 = b * b;
        println!("\nThe value of b is {b}.");
    }

    println!("\nThe value of b is {b}.");
}
