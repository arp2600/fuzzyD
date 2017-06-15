extern crate ignore;
extern crate crossbeam;

use std::env::home_dir;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use ignore::DirEntry;
use ignore::WalkBuilder;
use crossbeam::sync::MsQueue;

// This example demonstrates:
// Use WalkParallel to get all the paths above the home directory
// Fill a vector with the paths from WalkParallel as they come in
// Provide locked access to the vector of paths to do something with

fn main() {
    let queue: Arc<MsQueue<Option<DirEntry>>> = Arc::new(MsQueue::new());

    // Create a WalkParallel that fills queue with DirEntrys
    let walker_queue = queue.clone();
    thread::spawn(move || {
        let home = home_dir().unwrap();
        let walker = WalkBuilder::new(home).threads(4).build_parallel();
        walker.run(|| {
            let queue = walker_queue.clone();
            Box::new(move |result| {
                use ignore::WalkState::*;
                queue.push(Some(result.unwrap()));
                Continue
            })
        });
        walker_queue.push(None);
    });

    // Thread shared, blocking vector 
    let paths = Arc::new(Mutex::new(Vec::new()));

    // Create a thread to fill the vector with paths
    // as it receives them from the WalkParellel
    let fill_paths = paths.clone();
    thread::spawn(move || {
        while let Some(dent) = queue.pop() {
            // Lock the paths vector, add an entry, unlock
            let mut fill_paths = fill_paths.lock().unwrap();
            fill_paths.push(dent.path().to_path_buf());
        }
    });

    // Print the length of the vector every 250 milliseconds
    loop {
        thread::sleep(Duration::from_millis(250));

        // Lock paths to read the length
        // This would be where the fuzzy match algorithm happens
        let paths = paths.lock().unwrap();
        println!("Found {} paths", paths.len());
    }
} 
