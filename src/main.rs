use std::io;
fn main() {
    println!("Welcome to The Number Guessing Game ");
    println!("Guess a number ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read uer input");
    println!("You guessed {guess}");
}
