/**
 * To link a crate to this new library, the extern crate declaration must be used.
 * This will not only link the library, but also import all its items under a module named the same as the library.
 * The visibility rules that apply to modules also apply to libraries.
 */

// Link to `library`, import items under the `rary` module
// This example follows the library created in the file creates.rs
extern crate rary;

fn main() {
  rary::public_function();

  // Error! `private_function` is private
  //rary::private_function();

  rary::indirect_access();
}

/**
 * To run this example please run the commands showed below
 * # Where library.rlib is the path to the compiled library, assumed that it's
 * # in the same directory here:
 * rustc externCrate.rs --extern rary=library.rlib && ./externCrate
   called rary's `public_function()`
   called rary's `indirect_access()`, that
   > called rary's `private_function()`
 */

