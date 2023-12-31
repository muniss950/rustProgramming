extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
   println!("Guess the number"); 
   let secret_number=rand::thread_rng().gen_range(1..101);
   let mut tries=0;
   // println!("The scrent number is: {}",secret_number);
   loop{
   println!("Please input your Guess: ");
   let mut guess= String::new();
   io::stdin().read_line(&mut guess)
       .expect("Failed to read line");
   let guess:u32=guess.trim().parse()
       .expect("Please type a number");
   match guess.cmp(& secret_number){
       Ordering::Less=>println!("Too small!"),
       Ordering::Greater=>println!("Too big"),
       Ordering::Equal=>{
           println!("You guess the right number! after {} tries ,it's {}",tries,guess);
           break;
       }
   }
   tries+=1;
   }
}
