extern crate find;
use find::find;
use std::path::PathBuf;

fn main() {
    for path in find(PathBuf::from(".")) {
        println!("{:?}", path);
    }
}
