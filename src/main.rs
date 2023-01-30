// using standard io library
use std::io;

// no ! : this is a function, not a macro
fn main() {

    // ! : calling a macro, not a function
    println!("Guess the number!");
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

    // placeholders are neat
    println!("You guessed: {guess}");
    /*
    compare to
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
    */
}
