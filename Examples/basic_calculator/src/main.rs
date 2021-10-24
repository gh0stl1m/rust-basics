
fn main() {
    let user_values = get_user_input();
    let (number1, operator, number2) = user_values;
    let result = calculate(number1, operator, number2);

    println!("Result: {}", result);
}

fn get_user_input() -> (f32, String, f32) {

    let mut user_input = String::new();

    println!("Enter the operation that you want to perform: ");
    std::io::stdin().read_line(&mut user_input).expect("Failed to read operation");

    let input_parsed: Vec<&str> = user_input.split(" ").collect();

    let number1: f32 = input_parsed[0].trim().parse().expect("Failed to convert number 1. Please enter a valid number");
    let number2: f32 = input_parsed[2].trim().parse().expect("Failed to convert number 2. Please enter a valid number");
    let operator = input_parsed[1].trim().to_string();

    return (number1, operator, number2);
}

fn calculate(number1: f32, operator: String, number2: f32) -> f32 {
    match operator.as_ref() {
        "+" => number1 + number2,
        "-" => number1 - number2,
        "*" => number1 * number2,
        "/" => number1 / number2,
        _ => 0.0
    }
}
