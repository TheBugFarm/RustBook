use std::io;
fn main() {
    println!("The Number Guessing Game ");
    println!("Enter your guess ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read uer input");
    println!("You guessed {guess}");
}
