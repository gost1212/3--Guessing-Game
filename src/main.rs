use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_num: u32 = rand::thread_rng()
    .gen_range(1..=100);

    println!("Secret number is: {secret_num}");

    loop {

        println!("Enter a number: ");

        let mut guess = String::new();//create String variable

        io::stdin()//use IO
        .read_line(&mut guess)//read line
        .expect("Failed to read line.");//if error, print this
        //interesting way of coding

        /*let guess: u32 = guess
        .trim()
        .parse()
        .expect("Please type a number.");*/

        let guess: u32 = match guess
        .trim()
        .parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.\n");
                continue;
            }
        };

        match guess.cmp(&secret_num){
            Ordering::Greater => println!("Too Big.\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            },
            Ordering::Less => println!("Too Small.\n")
        }
        
    }
}
