extern crate ignore;
extern crate crossbeam;

use std::thread;
use std::sync::Arc;
use ignore::DirEntry;
use ignore::WalkBuilder;
use crossbeam::sync::MsQueue;


pub struct Find {
    queue: Arc<MsQueue<Option<DirEntry>>>,
} 


impl Find {
    fn new() -> Find {
        let queue: Arc<MsQueue<Option<DirEntry>>> = Arc::new(MsQueue::new());

        let walker_queue = queue.clone();
        thread::spawn(move || {
            let walker = WalkBuilder::new("./").threads(16).build_parallel();
            walker.run(|| {
                let queue = walker_queue.clone();
                Box::new(move |result| {
                    use ignore::WalkState::*;
                    println!("Pushing path to queue");
                    queue.push(Some(result.unwrap()));
                    Continue
                })
            });
            walker_queue.push(None);
        });

        Find {
            queue: queue,
        }
    }
}


impl Iterator for Find {
    type Item = DirEntry;

    fn next(&mut self) -> Option<DirEntry> {
        self.queue.pop()
    }
}


fn main() {
    let find = Find::new();
    thread::sleep(std::time::Duration::from_secs(3));
    for entry in find {
        println!("{}", entry.path().display());
        thread::sleep(std::time::Duration::from_millis(250));
    }
} 
