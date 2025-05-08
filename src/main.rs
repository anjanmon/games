use std::io;
use rand::Rng;
use std::cmp::Ordering;



fn guessing_game(){
    println!("Choose your  the number!");
    
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


fn rps(){

    println!("Welcome to a game of Rock, Paper and Scissors!!");
    

}



fn main(){
    println!("Choose your game -");

    println!("Press 1 for a Guessing Game or Press 2 for Rock Paper and Scissors");

    let mut game = String::new();
    
    io::stdin()
        .read_line(&mut game)
        .expect("Failed to read line!");

    match game.trim().parse::<u32>() {
            Ok(num) => {
                match num {
                    1 => guessing_game(),
                    2 => rps(),
                    _ => println!("Not supported yet!"),
                }
              }
            Err(_) => println!("Please enter a valid number."),
        };
}
