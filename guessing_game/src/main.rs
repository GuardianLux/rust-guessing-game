use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //Generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    //Loop until correct guess
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        //Read user input from stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            //Convert user input to unsigned 32bit int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        //Match guess using compare and ordering to determine result
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