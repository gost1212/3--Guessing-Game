use std::io;

fn main() {
    println!("Guess the number!");

    println!("Enter a number: ");

    let mut guess = String::new();//create String variable

    io::stdin()//use IO
    .read_line(&mut guess)//read line
    .expect("Failed to read line.");//if error, print this
    //interesting way of coding

    println!("You guessed: {guess}");
}
