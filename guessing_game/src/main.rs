use std::io; // import library io from standard library (std)

fn main() { // fn declares new function called main
    println!("Guess the number!"); // prints to screen
    println!("Please enter your guess: "); // prints to screen
    let mut guess = String::new(); // creating mutable variable called guess (vars are immutable by default)
    // let guess = String::new(); // immutable
    
    io::stdin().read_line(&mut guess) // call std function from io module to take in user input
        .expect("Something went wrong reading the line.");
    // read_line puts what the user types into the string we're passing it, but returns a value - in this case io::Result 
    
        println!("You guessed: {}", guess); // prints the string we saved the user's input in
}