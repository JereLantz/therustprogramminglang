use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number between 1 and 100");

    let secret_number: u32 = rand::random_range(1..=100);
    //println!("The secret number is: {secret_number}");
    let mut tries: u32 = 0;

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers!");
                continue
            },
        };

        println!("You guessed {guess}");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small."),
            Ordering::Greater => println!("Too big."),
            Ordering::Equal => {
                println!("Correct, you win! You made {tries} guesses.");
                break;
            }
        }
        tries += 1;
        println!("Guess again.");
    }
}
