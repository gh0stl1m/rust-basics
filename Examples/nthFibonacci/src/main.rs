use std::io;

fn nth_fibonacci(x: u32) -> u32 {
    if x <= 1 { return x }

    return nth_fibonacci(x - 1) + nth_fibonacci(x - 2);
}

fn main() {
    println!("Please enter a number: ");
    
    let mut fib_input = String::new();

    io::stdin()
        .read_line(& mut fib_input)
        .expect("Please enter a valid input");

    let fib_trimmed = fib_input.trim();
    let mut fib_number: u32 = 0;

    match fib_trimmed.parse::<u32>() {
        Ok(value) => fib_number = value,
        Err(..) => println!("Invalid number"),
    }

     fib_number = nth_fibonacci(fib_number);

    println!("The fibonacci number is {}", fib_number);
}
