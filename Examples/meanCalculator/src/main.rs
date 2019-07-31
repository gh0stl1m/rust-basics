use std::io;

fn main() {
    println!("Please enter the list of numbers separated by comma: ");
    let mut input_numbers = String::new();

    io::stdin()
        .read_line(& mut input_numbers)
        .expect("Please enter a valid list");
    
    let input_splitted: Vec<&str> = input_numbers
        .trim()
        .split(",")
        .collect();

    let mut mean: u32 = 0;
    for number in &input_splitted {
        let num_parsed: u32 = number.parse().unwrap();
        mean += num_parsed;
    }

    mean /= input_splitted.len() as u32;

    println!("Number list entered: {:?}", input_splitted);
    println!("Number list mean: {}", mean);
}
