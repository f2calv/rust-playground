use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess...");

        //let mut guess = String::new();
        let mut guess;
        guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // match io::stdin().read_line(&mut guess) {
        //     Ok(n) => {
        //         println!("{n} bytes read");
        //         println!("{guess}");
        //     }
        //     Err(error) => println!("error: {error}"),
        // }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed, {guess}");

        // let x = 5;
        // let y = 10;

        // println!("{x}");
        // println!("{y}");
        // println!("{x} and {y}");
        // println!("x = {} and y = {}", x, y);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
