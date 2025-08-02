mod trie;
mod words_reader;

use lazy_static::lazy_static;
use regex::Regex;
use trie::Trie;
use words_reader::WordIterator;

lazy_static! {
    static ref WORD_REGEX: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
}

fn main() {
    let mut trie = Trie::new();
    let word_reader = match WordIterator::new("./data/words.txt") {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Error reading words file: {}", e);
            return;
        }
    };
    for result in word_reader {
        match result {
            Ok(word) => {
                if !WORD_REGEX.is_match(&word) {
                    continue;
                }
                trie.insert(&word);
            }
            Err(_) => {
                continue;
            }
        }
    }

    let found = trie.find("example");
    println!("Word 'example' found: {}", found);

    let closest_words = trie.suggest("spple", 3);
    println!("Suggestions for 'epple': {:?}", closest_words);
}
