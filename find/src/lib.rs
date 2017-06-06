use std::fs::{self};
use std::path::PathBuf;
use std::vec::Vec;

pub struct Find {
    to_search: Vec<PathBuf>,
} 

impl Iterator for Find {
    type Item = PathBuf;

    fn next(&mut self) -> Option<PathBuf> {
        let opt_top = self.to_search.pop();
        if opt_top.is_none() {
            return opt_top;
        }
        let top = opt_top.unwrap();

        if top.is_dir() {
            for entry in fs::read_dir(&top).unwrap() {
                let path = entry.unwrap().path();
                self.to_search.push(path);
            }
        }
        Some(top)
    }
}

pub fn find(root: PathBuf) -> Find
{
    Find { to_search: vec![root,] }
}
