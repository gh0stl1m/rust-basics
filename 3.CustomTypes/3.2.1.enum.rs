/**
 * The enum keyword allows the creation of a type which may be one of a few different variants.
 * Any variant which is valid as a struct is also valid as an enum.
 */

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
  // An `enum` may either be `unit-like`,
  PageLoad,
  PageUnload,
  // like tuple structs,
  KeyPress(char),
  Paste(String),
  // or like structures.
  Click { x: i64, y: i64 },
}

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

// enum with implicit discriminator (starts at 0)
enum Number {
  Zero,
  One,
  Two,
}

// enum with explicit discriminator
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff,
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
      // Note the lack of scoping because of the explicit `use` above.
      Rich => println!("The rich have lots of money!"),
      Poor => println!("The poor have no money..."),
    }

    match work {
      Civilian => println!("Civilians work!"),
      Soldier  => println!("Soldiers fight!"),
    }

  // `enums` can be cast as integers.
  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);

  println!("roses are #{:06x}", Color::Red as i32);
  println!("violets are #{:06x}", Color::Blue as i32);
}