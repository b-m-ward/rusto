use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub const MAX_OPTS: u32 = 100000;


pub fn guessing_game(x: u32) {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1,x+1);

    //println!("The secret number is {}", secret_num);

    loop {
        println!("Please input a guess: ");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("YERP!");
                break;
            }
        }
    }

}