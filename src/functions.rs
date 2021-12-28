// use regex::Regex;
use std::io;
//-----------------------------------------------------------------------------

pub fn primary_function() {
    println!("Hello, world!");
    second_function();
    let a = sum(5, 6);
    println!("The sum of 5 and 6 is {}", a);
}

fn second_function() {
    println!("Another function.");
}

pub fn sum(x: i32, y: i32) -> i32 {
    let a = x + y;
    println!("the sum is {}", a);
    a
}

pub fn guess_number() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
