use std::io; // import library io from standard library (std)
use std::cmp::Ordering; // import library cmp from standard library (std)
use rand::Rng; // import library rand from std (used for random numbers)

fn main() { // fn declares new function called main
    println!("Guess the number!"); // prints to screen
    
    let secret_number = rand::thread_rng().gen_range(1, 101); // gen_range generates a random number between 1 and 100, and assigns it to secret_number
    println!("The secret is: {}", secret_number);
    println!("Please enter your guess: "); // prints to screen
    let mut guess = String::new(); // creating mutable variable called guess (vars are immutable by default)
    // let guess = String::new(); // immutable
    
    io::stdin().read_line(&mut guess) // call std function from io module to take in user input
        .expect("Something went wrong reading the line.");
    // read_line puts what the user types into the string we're passing it, but returns a value - in this case io::Result 
    
    println!("You guessed: {}", guess); // prints the string we saved the user's input in
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}