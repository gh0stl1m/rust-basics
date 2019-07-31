use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("¡Guess a number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Plase guess a number: ");

        let mut guess = String::new();

        io::stdin().read_line(& mut guess)
            .expect("Failed reading line!");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed the number: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            },
            Ordering::Greater => println!("too big!!"),
        }
    }
}
