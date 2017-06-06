extern crate find;
extern crate fuzz;

use std::env;
use std::io;

fn main() {
    use io::Write;

    print!("Enter search term: ");
    io::stdout().flush().unwrap();

    let mut needle = String::new();
    io::stdin().read_line(&mut needle).expect("Error reading input.");
    let needle = needle.trim();

    let current_working_directory = env::current_dir().unwrap();
    let finder = find::find(current_working_directory)
        .filter(|ref p| fuzz::substrings(&needle, p.to_str().unwrap()) != fuzz::Score(0));

    for path in finder {
        println!("Matched: {:?}", path);
    }
}

