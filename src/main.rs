mod words_reader;
mod trie;

use words_reader::WordIterator;

fn main() {
    let word_reader = WordIterator::new("./data/words.txt");
}
