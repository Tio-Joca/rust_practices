use rust_practices::*;

fn main () {
    //  Functions examples

    //  The functions used here were implemented in ../lib.rs source code
    //  After this line of comment, you'll find some function calls demonstrations

    //  Function calling (some_function)
    some_function();

    //  Function calling (print_value with 5 as its argument)
    print_value(5);

    //  Function calling (print_is_true with 5 and false as its arguments)
    print_is_true(5, false);

    //  Statements and expressions

    //  Statements are instructions to do some action and don't return a value
    //  Expressions are values and/or operations defined to evaluate to a resultant value
    //  Generally, statements contain expressions to perform the desired action
    //  Every line of code in Rust that ends with semicolon are statements

    let mut x: i32; //  A simple statement

    x = 3;  //  A statement with a simple expression

    println!("\nValue of x: {x}.");

    //  Some different kind of statement using a block scope

    //  To provide a value to the attribution operation, the last line of code inside the
    //  block of code doesn't have a semicolon. It makes possible to deliver the value to
    //  the variable, because the last line of code is an expression.
    x = {
        let power: i32 = x * x;
        power * power
    };

    println!("\nUpdated value of x: {x}.");

    let x: i32 = five();

    println!("\nThe value of x is {x}.");

    let x: i32 = plus_one(x);

    println!("\nThe value of x + 1 is {x}.");
}
