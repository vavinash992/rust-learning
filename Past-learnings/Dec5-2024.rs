use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let secret_number: u32 = rand::thread_rng().gen_range(1..=200);
    loop{
        let mut number = String::new();
        println!("Please enter the number:");
        io::stdin().read_line(&mut number)
                .expect("Error reading the number");
        let number: u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Entered value is not a number please enter a correct number");
                continue;
            }
        };
        match number.cmp(&secret_number){
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Less => println!("Too Small:("),
            Ordering::Equal => {println!("You won!!!!!!");
                                break}
        }
    }
}

// Code for info till https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html