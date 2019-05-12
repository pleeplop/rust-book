use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input a number:");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read lines");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
