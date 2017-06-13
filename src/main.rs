extern crate cli;

use std::io;

use cli::Canvas;

fn main() {
    use io::Read;

    // Create a canvas to write on that is 6 lines tall.
    let mut canvas = Canvas::new(6);

    // Write prompt on first line:
    canvas.write(0, "Enter search term: ");

    canvas.write(1, "1");
    canvas.write(2, "12");
    canvas.write(3, "123");
    canvas.write(4, "1234");
    canvas.write(5, "12345");

    // Just block until the next key pressed, for testing.
    let stdin = io::stdin();
    let mut bytes = stdin.bytes();
    let b = bytes.next().unwrap().unwrap();
}

