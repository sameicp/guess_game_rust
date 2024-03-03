use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, but got {}", value);
        }
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Do not spoil the fun and just enter a NUMBER!");
                continue;
            }
        };

        let user_guess: Guess = Guess::new(guess);

        let guess: i32 = user_guess.value();

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too high"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
