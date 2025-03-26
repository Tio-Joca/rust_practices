fn main () {
    let number: i128 = 3;

    if number < 5 {
        println!("The condition was true.");
    }   else {
        println!("The condition was false.");
    }

    if number != 0 {
        println!("The number {} is equivalent to true.", number);
    }   else {
        println!("The number {} is equivalent to false.", number);
    }

    if number % 4 == 0 {
        println!("The number {} is divisible by 4.", number);
    }   else if number % 3 == 0 {
        println!("The number {} is divisible by 3.", number);
    }   else if number % 2 == 0 {
        println!("The number {} is divisible by 2.", number);
    }   else {
        println!("The number {} isn't divisible by 2, 3, and 4.", number);
    }

    let condition: bool = true;

    let number: i64 = if condition { 1 } else { 0 };

    println!("The actual value of number is {}.", number);

    let mut number: i32 = 0;

    loop {
        println!("The current value of number is {}.", number);

        if number < 10 {
            number += 1;
            continue;
        }   else {
            break;
        }
    }

    let mut counter: i32 = 0;
    
    let result: i32 = loop {
        counter += 1;

        if counter > 9 {
            break counter * 2;
        }
    };

    println!("The result value is {}.", result);

    counter = 0;

    'counting_up: loop {
        println!("counter == {}", counter);
        let mut remaining: i32 = 10;

        loop {
            println!("remaining == {}", remaining);

            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }

    println!("The counter was ended with {} as its value.", counter);

    let mut number: i32 = 3;

    while number > 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let mut vector: [i128; 5] = [10, 20, 30, 40, 50];

    let mut counter: usize = 0;

    while counter < 5 {
        println!("The value is: {}.", vector[counter]);
        counter += 1;
    }

    vector = [20, 40, 60, 80, 100];

    for element in vector {
        println!("The value is: {}.", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!");
}