extern crate fuzz;

fn main() {
    // Keep taking user input

    let haystack = "the quick brown fox jumped over the lazy dog.";
    println!("Calculate fuzz-score from the string \"{}\":", haystack);

    let mut needle = String::new();

    loop {
        print!("> ");

        needle.clear();
        match std::io::stdin().read_line(&mut needle) {
            Ok(_) => {
                let score = fuzz::substrings(&needle.trim(), &haystack);
                println!("{:?}", score);
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}

