use std::cmp::Ordering;
use std::io;

use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guessing Game");

    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("The random number generated is {}", random_number);

    loop {
        println!("Please enter an input");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");
        println!("You guessed  : {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            },
            Ordering::Greater => println!("{}", "too big!".red()),
            Ordering::Less => println!("{}", "too small".red()),
        }
    }
}

/*
Here's what each part does:

let mut guess = String::new();
This is like saying, "Hey, I have a secret word, and I'm going to name it 'guess'. Right now, it's an empty secret,
like an empty treasure chest." let mut is used to create something that you can change later. In our case, 'guess'
is the name of our secret word. String::new() means your secret word is made of letters and not numbers, and
it starts empty.

io::stdin()
This part tells your computer, "Hey, I'm going to use your microphone to listen to something."
io stands for 'input/output', which is like saying "listening and talking" for the computer.
stdin() is the computer's way of using its microphone to listen to what you'll say (your input).
.read_line(&mut guess)

Now, this is like telling the robot, "Whatever you hear, put it inside our 'guess' treasure chest."
.read_line() is the robot's command to start listening and then store what it hears.
&mut guess means "take the 'guess' chest, and remember you can change what's inside it based on what you hear."
.expect("Failed to readline");

This is like a safety measure. It tells the robot, "If you have trouble listening or something goes wrong, let me
know by saying 'Failed to readline'."So, put it all together, it's like telling your robot: "Listen carefully and
store what you hear in the 'guess' chest. If you can't hear anything, let me know."
This is a way for your computer to get and remember what someone types!
*/
