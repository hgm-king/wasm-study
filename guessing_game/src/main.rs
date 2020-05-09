use std::io;

fn main() {
    println!("Hello, world!");

    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error, the line must not be empty!");

    println!("You guessed {}", guess);
}
