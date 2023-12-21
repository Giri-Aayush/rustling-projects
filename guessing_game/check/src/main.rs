use std::io;

fn main(){
    println!("Guessing Game self practice-time");
    let mut x = String::new();
    println!("Enter an input for the guess");
    io::stdin().read_line(&mut x).expect("Failed to readline");
    println!("The thing you entered is {}", x);
}