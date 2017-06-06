extern crate find;
extern crate fuzz;

use std::env;
use std::io;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::path::PathBuf;

use fuzz::{substrings, Score};

#[derive(Debug, Eq, PartialEq)]
struct Matched {
    score: Score,
    path: PathBuf,
}

impl Ord for Matched {
    fn cmp(&self, other: &Matched) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Matched {
    fn partial_cmp(&self, other: &Matched) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    use io::Write;

    print!("Enter search term: ");
    io::stdout().flush().unwrap();

    let mut needle = String::new();
    io::stdin().read_line(&mut needle).expect("Error reading input.");
    let needle = needle.trim();

    let current_working_directory = env::current_dir().unwrap();
    let mut heap: BinaryHeap<_> = find::find(current_working_directory)
        .map(|p| Matched{ score: substrings(&needle, p.to_str().unwrap()), path: p})
        .filter(|s| s.score != Score(0))
        .collect();

    // Print out the top ten results.
    for _ in 0..10 {
        let result = heap.pop().unwrap();
        println!("{:?}", result)
    }
}

