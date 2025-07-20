use std::fs::File;
use std::io::{BufRead, BufReader};


/*
 The idea over here was to create a python-like generator that can be used to read words from a file.
*/

pub struct WordIterator {
    reader: std::io::Lines<BufReader<File>>, // This holds the current position!
}
impl WordIterator {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file).lines();
        Ok(WordIterator { reader })
    }
}

impl Iterator for WordIterator{
    type Item = Result<String, std::io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next() // Uses stored state
    }
}