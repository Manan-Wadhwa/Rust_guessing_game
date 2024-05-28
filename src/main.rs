use std::io;

fn main(){
    println!("Welcome to the guessing");
    println!("Enter  your guess here");
    let mut guess =  String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Error: Could not read line");
    println!("Your Guess is {guess}");
}