use rand::Rng;
use regex::Regex;
use std::cmp::Ordering;
use std::io;
//-----------------------------------------------------------------------------

pub fn primary_function() {
    println!("Hello, world!");
    second_function();
    let a = sum(5, 6);
    println!("The sum of 5 and 6 is {}", a);
}
//-----------------------------------------------------------------------------

fn second_function() {
    println!("Another function.");
}
//-----------------------------------------------------------------------------

pub fn sum(x: i32, y: i32) -> i32 {
    let a = x + y;
    println!("the sum is {}", a);
    a
}
//-----------------------------------------------------------------------------

pub fn guess_number() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
//-----------------------------------------------------------------------------

pub fn generate_random_number() {
    println!("Generate a random number.");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("The secret number is: {}", secret_number);
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
//-----------------------------------------------------------------------------

pub fn raindrops(n: u32) -> String {
    let mut raindrop = String::new();

    raindrop
}
