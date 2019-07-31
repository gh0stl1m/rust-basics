/**
 * The for in construct can be used to iterate through an Iterator.
 * One of the easiest ways to create an iterator is to use the range notation a..b. This yields values from a (inclusive) to b (exclusive) in steps of one.
 * 
 * Alternatively, a..=b can be used for a range that is inclusive on both ends. The above can be written as:
 */

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
  for n in 1..101 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }

  for n in 1..=100 {
    if n % 15 == 0 {
      println!("fizzbuzz");
    } else if n % 3 == 0 {
      println!("fizz");
    } else if n % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}", n);
    }
  }
}

/**
 * The for in construct is able to interact with an Iterator in several ways.
 * As discussed in with the Iterator trait, if not specified,
 * the for loop will apply the into_iter function on the collection provided to convert the collection into an iterator.
 * 
 * This is not the only means to convert a collection into an iterator however, the other functions available include iter and iter_mut.
 * 
 * These 3 functions will return different views of the data within your collection.
 * iter - This borrows each element of the collection through each iteration.
 * Thus leaving the collection untouched and available for reuse after the loop.
 * fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
  }
 * into_iter - This consumes the collection so that on each iteration the exact data is provided.
 * Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
 * fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
  }
 *
 * iter_mut - This mutably borrows each element of the collection, allowing for the collection to be modified in place.
 * fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("Names: {:?}", names);
  }
 */