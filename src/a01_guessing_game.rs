use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub(crate) fn guess() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_num}");

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new();

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Fail to read line");

        println!("Your guess is {guess_str}");

        let guess_num: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess_num.cmp(&secret_num) {
            Ordering::Less => { println!("Too small!"); }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => { println!("Too big!"); }
        }
    }
}