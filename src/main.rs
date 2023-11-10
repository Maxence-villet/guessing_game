use core::time;
use std::{io, cmp::Ordering};
use rand::Rng;
use std::thread;

fn main() {

    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut score: u8 = 0; 

    loop {

        score = score + 1;

        println!("Please input your guess!");
    
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read Line!");
    
        
    
        println!("You guessed {}", guess);
    
        let guess: u32 = match guess.trim().parse() {

            Ok(num) => num,
            Err(_) => continue,

        };
    
        match guess.cmp(&secret_number)
        {
    
            Ordering::Less => println!("{}","Too small!"),
            Ordering::Greater => println!("{}", "Too big!"),
            Ordering::Equal => {
                println!("{}", "You win!");
                let sec = time::Duration::from_secs(5);
                thread::sleep(sec);
                break;
            },
    
        }

    } //loop

    let mut score_print: String = String::from("Your score is : ");
    score_print.push_str(&score.to_string());
    println!("{}",score_print);

}
