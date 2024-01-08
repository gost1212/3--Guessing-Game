use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num: u32 = rand::thread_rng()
    .gen_range(1..=100);

    print!("Secret number is: {secret_num}\n");

    println!("Enter a number: ");

    let mut guess = String::new();//create String variable

    io::stdin()//use IO
    .read_line(&mut guess)//read line
    .expect("Failed to read line.");//if error, print this
    //interesting way of coding

    let guess: u32 = guess
    .trim()
    .parse()
    .expect("Please type a number.");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_num){
        Ordering::Greater => print!("Too Big."),
        Ordering::Equal => print!("You win!"),
        Ordering::Less => print!("Too Small.")
    }
}
