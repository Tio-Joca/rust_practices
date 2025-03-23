//  Functions without return

pub fn some_function () {
    println!("\nThis is printed by a statement inside the function some_function in lib.rs source code!");
}

pub fn print_value (value: i32) {
    println!("\nThe value of the argument given to this function call is {value}.");
}

pub fn print_is_true (integer: i32, boolean: bool) {
    if (integer != 0) == boolean {
        println!("\nThe value {integer} is equal to {boolean}.");
    }   else {
        println!("\nThe value {integer} is not equal to {boolean}.");
    }
}

//  Functions with return

pub fn five () -> i32 {
    return 5;
}

pub fn plus_one (value: i32) -> i32 {
    return value + 1;
}