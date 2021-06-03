/**
 * Variable bindings have a scope, and are constrained to live in a block.
 * A block is a collection of statements enclosed by braces {}. Also, variable shadowing is allowed.
 * 
 * It's possible to declare variable bindings first, and initialize them later. However, this form is seldom used, as it may lead to the use of uninitialized variables.
 */

fn main() {
  // This binding lives in the main function
  let long_lived_binding = 1;

  // This is a block, and has a smaller scope than the main function
  {
    // This binding only exists in this block
    let short_lived_binding = 2;

    println!("inner short: {}", short_lived_binding);

    // This binding *shadows* the outer one
    let long_lived_binding = 5_f32;

    println!("inner long: {}", long_lived_binding);
  }
  // End of the block

  // Error! `short_lived_binding` doesn't exist in this scope
  // println!("outer short: {}", short_lived_binding);
  // FIXME ^ Comment out this line

  println!("outer long: {}", long_lived_binding);
  
  // This binding also *shadows* the previous binding
  let long_lived_binding = 'a';
  
  println!("outer long: {}", long_lived_binding);

  // Declare a variable binding
  let a_binding;

  {
    let x = 2;

    // Initialize the binding
    a_binding = x * x;
  }

  println!("a binding: {}", a_binding);

  let another_binding;

  // Error! Use of uninitialized binding
  // println!("another binding: {}", another_binding);
  // FIXME ^ Comment out this line

  another_binding = 1;

  println!("another binding: {}", another_binding);
}