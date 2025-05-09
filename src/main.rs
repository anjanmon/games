use std::io;
use rand::Rng;
use std::cmp::Ordering;
use rand::seq::IndexedRandom; 


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

    fn rps_comp<'a>(computer: &'a str, user: &'a str) -> &'a str {

        let computer = computer.to_lowercase(); 
        let user = user.to_lowercase();  

        match  (computer.as_str(), user.as_str()) {
            (a,b) if a==b => "It's a draw!.",

            ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => "Computer wins :(",
            ("scissors", "rock") | ("rock", "paper") | ("paper", "scissors") => "You win !!",
            _ => "Invalid Comparison",
        }
    }


    let options = vec!["Rock", "Paper", "Scissors"];
    let mut choice_index = rand::rng().random_range(0..=3);
    let mut computer_choice =  options[choice_index];



    
    println!("Welcome to a game of Rock, Paper and Scissors!!");
    println!("Please select the number of rounds you want to play:");
    
    let rounds: u32 = loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a valid number:"),
        }
    };


    println!("Choose your move - (Rock, Paper or Scissors) ");

    let mut user_move = String::new();

    io::stdin()
        .read_line(&mut user_move)
        .expect("Failed to register move!");
    
    let user_move = user_move.trim();

    let mut game_outcome: &str =   rps_comp(&computer_choice, &user_move);


    println!("Computer chose - {} . {}.", computer_choice, game_outcome);



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
