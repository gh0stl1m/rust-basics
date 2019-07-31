use std::io;

const VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

fn is_vowel(s: &str) -> bool {
    VOWELS.contains(&s)
}

fn pig_latin_conversor(s: &str) -> String {
    if is_vowel(&s[0..1]) {
        return format!("{}-hay", s);
    }

    return format!("{}-{}ay", &s[1..], &s[0..1]);
}

fn main() {
    println!("Please enter a word: ");
    let mut input_word = String::new();

    io::stdin()
        .read_line(& mut input_word)
        .expect("Please enter a valid word");
    
    input_word = input_word.trim().to_string();

    println!("Word: {}", input_word);
    println!("Pig latin word: {}", pig_latin_conversor(&input_word));
}