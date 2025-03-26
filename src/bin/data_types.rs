fn main () {
    // Scalar types

    // Integers with signals
    let a: i8 = 1;
    let b: i16 = 10;
    let c: i32 = 100; // Default for integers
    let d: i64 = 1000;
    let e: i128 = 100000;

    println!("Value of a: {a}.");
    println!("Value of b: {b}.");
    println!("Value of c: {c}.");
    println!("Value of d: {d}.");
    println!("Value of e: {e}.");

    // Integers without signals
    let f: u8 = 255;
    let g: u16 = 256;
    let h: u32 = 23455;
    let i: u64 = 30254543;
    let j: u128 = 230475289592;

    println!("\nValue of f: {f}.");
    println!("Value of g: {g}.");
    println!("value of h: {h}.");
    println!("Value of i: {i}.");
    println!("Value of j: {j}.");

    // Floating-point numbers
    let k: f32 = 3.14;  // Floating-point numbers
    let l: f64 = 1.125; // Floating-point numbers with duplicated precision (Default)

    println!("\nValue of k: {k}.");
    println!("Value of l: {l}.");

    // Arithmetic operations (using integers)

    let mut result: i128;

    // Addition (3 + 2)
    result = 3 + 2;
    println!("\n3 + 2 == {result}.");

    // Subtraction (3 - 2)
    result = 3 - 2;
    println!("3 - 2 == {result}.");

    // Multiplication (3 * 2)
    result = 3 * 2;
    println!("3 * 2 == {result}.");

    // Division (3 / 2)
    result = 5 / 2;
    println!("3 / 2 == {result}.");

    // Remainder (3 % 2)
    result = 3 / 2;
    println!("3 % 2 == {result}.");

    // Booleans
    let mut boolean: bool;

    boolean = true;
    println!("\nboolean == {boolean}.");

    boolean = false;
    println!("boolean == {boolean}.");

    // Characters
    let character: char;

    character = 'A';
    println!("\nValue of character: {character}.");

    // Tuples
    let tup: (i32, i64, bool);

    tup = (4, 2, true);

    let (x, y, z) = tup;
    println!("\nValue of the first value inside the tup: {x}.");
    println!("Value of the second value inside the tup: {y}.");
    println!("Value of the third value inside the tup: {z}.");

    let x: i32 = tup.0;
    let y: i64 = tup.1;
    let z: bool = tup.2;
    println!("\nValue of the first value inside the tup: {x}.");
    println!("Value of the second value inside the tup: {y}.");
    println!("Value of the third value inside the tup: {z}.");

    // Arrays
    let vector: [i128; 5] = [1, 2, 3, 4, 5];

    let a: i128 = vector[0];
    let b: i128 = vector[1];
    let c: i128 = vector[2];
    let d: i128 = vector[3];
    let e: i128 = vector[4];
    println!("\nValue of vector[0]: {a}.");
    println!("Value of vector[1]: {b}.");
    println!("Value of vector[2]: {c}.");
    println!("Value of vector[3]: {d}.");
    println!("Value of vector[4]: {e}.");

    let vector: [&str; 5] = ["copy", "paste", "code", "compile", "deploy"];

    let a: &str = vector[0];
    let b: &str = vector[1];
    let c: &str = vector[2];
    let d: &str = vector[3];
    let e: &str = vector[4];
    println!("\nValue of vector[0]: {a}.");
    println!("Value of vector[1]: {b}.");
    println!("Value of vector[2]: {c}.");
    println!("Value of vector[3]: {d}.");
    println!("Value of vector[4]: {e}.");

    let mut vector: [i64; 5] = [0; 5];

    vector[0] = 10;
    vector[1] = vector[0] * 2;
    vector[2] = vector[1] * 3;
    vector[3] = vector[2] * 4;
    vector[4] = vector[3] * 5;

    let a: i64 = vector[0];
    let b: i64 = vector[1];
    let c: i64 = vector[2];
    let d: i64 = vector[3];
    let e: i64 = vector[4];

    println!("\nValue of vector[0]: {a}.");
    println!("Value of vector[1]: {b}.");
    println!("Value of vector[2]: {c}.");
    println!("Value of vector[3]: {d}.");
    println!("Value of vector[4]: {e}.");
}
