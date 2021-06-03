/**
 * Variable bindings are immutable by default, but this can be overridden using the mut modifier.
 */

fn main() {
  let _immutable_binding = 1; // When you use as a prefix _ in a variable the compiler omits the unuser var warning
  let mut mutable_binding = 1;

  println!("Before mutation: {}", mutable_binding);

  // Ok
  mutable_binding += 1;

  println!("After mutation: {}", mutable_binding);
}