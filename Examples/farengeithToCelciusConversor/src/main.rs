use std::io;

fn main() {
    println!("Please enter the temperature in fahrenheit that you want to convert: ");

    let mut temperatureInFInput = String::new();

    io::stdin()
        .read_line(& mut temperatureInFInput)
        .expect("Please enter a value");

    let temperatureTrimmed = temperatureInFInput.trim();
    let mut temperature: f32 = 0.0;


    match temperatureTrimmed.parse::<f32>() {
        Ok(value) => temperature = value,
        Err(..) => println!("You entered a invalid value, please enter a number"),
    }

    // Casting
    temperature = (temperature - 32.0) * (5.0 / 9.0);

    println!("The temperature in celcius is {}", temperature);
}
