use std::io;
use rand::Rng; // cargo doc --open
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    
    let secret_number = rand::thread_rng().gen_range(1,100);
    println!("Secret number {}", secret_number);
    
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = guess.trim()
                            .parse()
                            .expect("Please type a number!");

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("{}You guessed it!{}", "\x1b[32m", "\x1b[0m");
                break;
            }
        }
    }
}
