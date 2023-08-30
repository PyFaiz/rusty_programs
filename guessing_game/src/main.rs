use core::num;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to Guess the Number Game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Guess a Number :");

        let mut guess  = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You Guessed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    //print!("Random Number is : {secret_number}")

    }
    
}