use std::io;

pub(crate) fn guess() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess_str = String::new();

    io::stdin()
        .read_line(&mut guess_str)
        .expect("Fail to read line");

    println!("Your guess is {guess_str}")
}