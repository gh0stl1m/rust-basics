use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.width < self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn parse_input (s: &str) -> u32 {
    let input_trimmed = s.trim();
    let mut input_parsed = 0;

    match input_trimmed.parse::<u32>() {
        Ok(value) => input_parsed = value,
        Err(..) => println!("Error parsing data"),
    }

    return input_parsed;
}

fn main() {
    println!("Please enter the width of the rectangle: ");
    let mut width_input = String::new();

    io::stdin()
        .read_line(& mut width_input)
        .expect("PLease enter the widh of the rectangle");
    
    let width_value = parse_input(&width_input);

    println!("Please enter the height of the rectangle: ");
    let mut height_input = String::new();

    io::stdin()
        .read_line(& mut height_input)
        .expect("Please enter the height of the rectangle");
    
    let height_value = parse_input(&height_input);

    let new_rectangle = Rectangle { width: width_value, height: height_value };

    println!("The rectangle with width {} and height {} has an area of {}", new_rectangle.width, new_rectangle.height, new_rectangle.area());

    let new_rectangle2 = Rectangle { width: 20, height: 30 };

    println!("The rectangle 2 can completly fit into the rectangle that you entered? {}", new_rectangle.can_hold(&new_rectangle2));

    let new_square = Rectangle::square(20);

    println!("The square dimensions are: {}w and {}h", new_square.width, new_square.height);
}
