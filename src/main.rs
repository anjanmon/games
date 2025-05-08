use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    println!("Guess the number!");
    
    let secret_number = rand::rng().random_range(1..=100);
    
 
    loop {
    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin() // can also be called by std::io::stdin
        .read_line(&mut guess) //& indicates that the guess argument is a reference // References are immutable by default hence, &mut rather than &guess
        .expect("Failed to read line"); // Handling potential failure // Best practice to introduce a newline when calling a method


    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
   
    
    println!("You guessed: {}", guess); //read_line returns a Result value. It is an enumeration (enum). Each possible state of enum -> variant. The variants of results are Ok and Err. Result type has expect method defined on it
        
        match guess.cmp(&secret_number){
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => { println!("You win!");
            break;
            }
        }
    }
}