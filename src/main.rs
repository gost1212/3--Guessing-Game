use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secretNum = rand::thread_rng()
    .gen_range(1..=100);

    println!("Enter a number: ");

    let mut guess = String::new();//create String variable

    io::stdin()//use IO
    .read_line(&mut guess)//read line
    .expect("Failed to read line.");//if error, print this
    //interesting way of codingcr

    println!("You guessed: {guess}");
}
