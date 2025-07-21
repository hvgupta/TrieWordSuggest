use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode { 
    pub letter: char,
    pub is_word: bool,
    pub children: HashMap<char, TrieNode>
}
impl TrieNode{
    pub fn new(letter: char, is_word: bool) -> Self {
        TrieNode {
            letter,
            is_word,
            children: HashMap::new(),
        }
    }
}