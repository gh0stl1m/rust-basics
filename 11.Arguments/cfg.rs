/**
 * Conditional compilation is possible through two different operators:
 * 
 * the cfg attribute: #[cfg(...)] in attribute position
 * the cfg! macro: cfg!(...) in boolean expressions
 * 
 * Both utilize identical argument syntax.
 * 
 * Some conditionals like target_os are implicitly provided by rustc, but custom conditionals must be passed to rustc using the --cfg flag.
 * #[cfg(some_condition)]
  fn conditional_function() {
    println!("condition met!");
  }

  fn main() {
    conditional_function();
  }
 * command: rustc --cfg some_condition custom.rs && ./custom
 */

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
  println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
  println!("You are *not* running linux!");
}

fn main() {
  are_you_on_linux();
  
  println!("Are you sure?");
  if cfg!(target_os = "linux") {
    println!("Yes. It's definitely linux!");
  } else {
    println!("Yes. It's definitely *not* linux!");
  }
}