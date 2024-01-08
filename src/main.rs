use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num: u32 = rand::thread_rng()//defning an unsigned 32bit value, with a generated 1..100 number.
    .gen_range(1..=100);

    println!("Secret number is: {secret_num}");

    loop {

        println!("Enter a number: ");

        let mut guess = String::new();//create String variable

        io::stdin()//use IO
        .read_line(&mut guess)//read line
        .expect("Failed to read line.");//if input read failed, print this
        //interesting way of coding

        //convert string input to number
        let guess: u32 = match guess
        .trim()
        .parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.\n");
                continue;
            }
        };

        //compare input number with secret number
        match guess.cmp(&secret_num){
            Ordering::Greater => println!("Too Big.\n"),//if input is bigger
            Ordering::Less => println!("Too Small.\n"),//if input is smaller
            Ordering::Equal => {//if input is equal
                println!("You win!\n");
                break;
            }
        }
        
    }
}
