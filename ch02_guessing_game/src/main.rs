use std::io;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Guess the number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");
        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("You Win!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Less => println!("Too small!"),
        }
    }
}
