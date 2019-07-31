use std::io;
use std::collections::HashMap;

fn main() {
    println!("Please enter a text: ");
    let mut input_text = String::new();

    io::stdin()
        .read_line(& mut input_text)
        .expect("Please enter a valid text");

    let mut words_map = HashMap::new();

    for word in input_text.split_whitespace() {
        let count = words_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Words in text: {:?}", words_map);
}
