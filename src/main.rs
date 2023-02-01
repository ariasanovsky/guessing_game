// using standard io library
use std::io;

// using the rand library
use rand::Rng;

// smart comparisons with enums
use std::cmp::Ordering;

// no ! : this is a function, not a macro
fn main() {

    // ! : calling a macro, not a function
    println!("Guess the number!");

    // first line in initializing the RNG
    let secret_number = rand::thread_rng()
        .gen_range(1..=100);
    // second line is calling it with a range
    // if I let oneHundred = 1..=100; then oneHundred is an int range

    // println!("The secret number is: {secret_number}");
    
    loop{
        println!("Please input your guess.");

        // let : creates variable
        // mut : makes it mutable (immut by default)
        // =   : immediately binds to the the rvalue
        let mut guess = String::new();

        // reads line and accounts for error message
        io::stdin()                             // calls std library io function
            .read_line(&mut guess)              // returns a Result   : an enum Ok or Err
            .expect("Failed to read line");     // handles the Result : what to say for an Err
                                                // compiler warning for unhandled Result, better practice
        // references in rust can be mutable (default immutable)

        // reuses variable name
        let guess: u32 = match guess.trim()     // returns str : whitespace trimmed on ends
            .parse()                            // returns Result : parsed to the desired type (u32)?
            {
                Ok(num) => num,                 // on successful match, captures as num and returns num
                Err(_)  => {                    // else, continues the loop (I added a print)
                    println!("invalid guess!");
                    continue;
                }
            };
    
        // a match has arms (patterns to match)
        match guess.cmp(&secret_number){
            Ordering::Less      => println!("Too small!"), 
            Ordering::Greater   => println!("Too large!"), 
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}
