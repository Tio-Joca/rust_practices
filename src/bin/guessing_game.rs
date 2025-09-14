use std::io::{Write, stdin, stdout};
use rand::Rng;
use std::cmp::Ordering;

fn main ()  {
    println!("Welcome to the guessing game!\n\n\
    Try to guess the number from 1 to 100.\n");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess: String = String::new();
        print!("Please input your guess here: ");
        stdout()
            .flush()
            .expect("\nFailed to flush stdout");

        stdin()
            .read_line(&mut guess)
            .expect("\nFailed to read line.");

        let guess: i32 = match guess.trim().parse::<i32>()  {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("\nToo small."),
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            },
            Ordering::Greater => println!("\nToo big.")
        }
    }
}
