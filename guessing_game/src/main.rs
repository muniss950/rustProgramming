extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let mut guess_times=0;
    let secret_number=rand::thread_rng().gen_range(1..101);
    loop{
    println!("Guess the number:");
    let mut guess=String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // println!("You guessed: {}",guess);
    // println!("Secret number: {}",secret_number);
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
        
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
            println!("You win,the number is {} after {} guesses",guess,guess_times);
            break;
        }
    }
    guess_times+=1;
    }
}
