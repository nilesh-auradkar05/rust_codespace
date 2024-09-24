// import I/O from standary library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); /* thread_rng returns random number generator that is local to the current thread.
                                                               gen_range is associated funtion to rand class. that returns random number
                                                               from random number generator.*/

    println!("Please input your number.");

    // let helps to declare variables.
    // mut indicates the variable guess is mutable
    // String::new() is a function that returns new instance of String class.
    // :: indicates that new is associated function of the String class.
    let mut guess = String::new();

    //::stdin is associated function of io class to accepts input from user.
    io::stdin()
        .read_line(&mut guess) /* read_line(&mut guess) gets the input from the terminal and appends it to guess variable WITHOUT OVERRIDING it's previous content.
        The & indicates that the argument is reference which let multiple parts of the code, access one piece of data without needing
        to copy that data into memory multiple times,*/
        .expect("Failed to read line!"); /* To handle any exceptions raised by the input library or String class */

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Guessed number is lower than actual number!"),
        Ordering::Greater => println!("Guessed number is greater than actual number!"),
        Ordering::Equal => println!(
            "Great! you guessed it right. The number was {}",
            secret_number
        ),
    }

    println!("You guessed {}", guess);
}
