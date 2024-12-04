/*
- `println!` calls a Rust macro. If it had called a function instead, it would be entered as `println` (without the `!`).
 Using a `!` means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.
- In windows we get an extra file with extension ".pdb" which is a program database file. It contains debugging information that is very useful for debugging.
 */

// fn main() {
//     println!("Hello, world!")
// }

/*
- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude, and you can see everything in it in the standard library documentation.
If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.
*/

use std::io;

fn main() {
    let mut number = String::new();
    println!("Hi please enter the number:");
    io::stdin().read_line(&mut number)
                .expect("Failed to read the line"); // read_line takes whatever the user types into standard input and append that into a string (without overwriting its contents)
    println!("The number you guessed is {}",number);

}

// Read till https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-random-number 