use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn run() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");

    println!("Guessing number game!");

    loop {
        let mut guess: String = String::new();

        println!("Input your guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Cannot read the input you typed");

        let mut guess = match guess.trim().parse::<i32>() {
            Ok(v) => v,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Guess bigger"),
            Ordering::Greater => println!("Guess lower"),
            Ordering::Equal => {
                println!("You found it!!!");
                break;
            },
        }
    }

}