/**
 * Rust provides pattern matching via the match keyword, which can be used like a C switch
 * 
 * A match block can destructure items in a variety of ways.
 * Destructuring Enums
 * Destructuring Pointers
 * Destructuring Structures
 * Destructuring Tuples
 */

#[allow(dead_code)]
enum Color {
  // These 3 are specified solely by their name.
  Red,
  Blue,
  Green,
  // These likewise tie `u32` tuples to different names: color models.
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32),
}

fn main() {
  let number = 13;
  // TODO ^ Try different values for `number`

  println!("Tell me about {}", number);
  match number {
    // Match a single value
    1 => println!("One!"),
    // Match several values
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    // Match an inclusive range
    13...19 => println!("A teen"),
    // Handle the rest of cases
    _ => println!("Ain't special"),
  }

  let boolean = true;
  // Match is an expression too
  let binary = match boolean {
    // The arms of a match must cover all the possible values
    false => 0,
    true => 1,
    // TODO ^ Try commenting out one of these arms
  };

  println!("{} -> {}", boolean, binary);

  // In the example below you can see how you can destructure tuples
  let pair = (0, -2);

  println!("Tell me about {:?}", pair);
  // Match can be used to destructure a tuple
  match pair {
    // Destructure the second
    (0, y) => println!("First is `0` and `y` is `{:?}`", y),
    (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
    _      => println!("It doesn't matter what they are"),
    // `_` means don't bind the value to a variable
  }

  // In the example below you can see how you can destructure enums
  let color = Color::RGB(122, 17, 40);

  println!("What color is it?");
  // An `enum` can be destructured using a `match`.
  match color {
    Color::Red   => println!("The color is Red!"),
    Color::Blue  => println!("The color is Blue!"),
    Color::Green => println!("The color is Green!"),
    Color::RGB(r, g, b) =>
        println!("Red: {}, green: {}, and blue: {}!", r, g, b),
    Color::HSV(h, s, v) =>
        println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
    Color::HSL(h, s, l) =>
        println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
    Color::CMY(c, m, y) =>
        println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
    Color::CMYK(c, m, y, k) =>
        println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k),
    // Don't need another arm because all variants have been examined
  }

  // In the example below you can see how you can destructure structs
  struct Foo { x: (u32, u32), y: u32 }

  // destructure members of the struct
  let foo = Foo { x: (1, 2), y: 3 };
  let Foo { x: (a, b), y } = foo;

  println!("a = {}, b = {},  y = {} ", a, b, y);

  // you can destructure structs and rename the variables,
  // the order is not important

  let Foo { y: i, x: j } = foo;
  println!("i = {:?}, j = {:?}", i, j);

  // and you can also ignore some variables:
  let Foo { y, .. } = foo;
  println!("y = {}", y);

  // this will give an error: pattern does not mention field `x`
  // let Foo { y } = foo;
}