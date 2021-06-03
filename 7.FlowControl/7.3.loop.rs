/**
 * Rust provides a loop keyword to indicate an infinite loop.
 * The break statement can be used to exit a loop at anytime, whereas the continue statement can be used to skip the rest of the iteration and start a new one.
 * 
 * It's possible to break or continue outer loops when dealing with nested loops.
 * In these cases, the loops must be annotated with some 'label, and the label must be passed to the break/continue statement.
 */

fn main() {
  let mut count = 0u32;

  println!("Let's count until infinity!");

  // Infinite loop
  loop {
    count += 1;

    if count == 3 {
      println!("three");

      // Skip the rest of this iteration
      continue;
    }

    println!("{}", count);

    if count == 5 {
      println!("OK, that's enough");

      // Exit this loop
      break;
    }
  }

  // TODO Please comment the before loop ^^ to run this example
  'outer: loop {
    println!("Entered the outer loop");

    'inner: loop {
      println!("Entered the inner loop");

      // This would break only the inner loop
      //break;

      // This breaks the outer loop
      break 'outer;
    }
    println!("This point will never be reached");
  }

  println!("Exited the outer loop");

  // TODO Please comment the before loop ^^ to run this example
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The counter value is -> {} and the result -> {}", counter, result);
}