// guess.rs

use std::io;
use std::cmp::Ordering;

fn main() {
    let secret : u32 = 42;
    println!("The secret is: {}", secret);

    loop {
        println!("Enter a guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess!");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Got it!");
                break;
            }
        };
    }
}