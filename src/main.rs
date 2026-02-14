use rand::RngExt;
use std::cmp::Ordering;
use std::io;
fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    let mut guess_time = 0;
    print!("{}", secret_number);
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    //let guess = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);
    let secret_string=String::from("hello");
    match guess.cmp(&secret_string) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
