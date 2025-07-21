mod node;
mod suggest;

use node::TrieNode;

#[derive(Debug)]
pub struct Trie {
    root: TrieNode,
    max_word_diff: u8,
}
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new('\0', false), // Root node with no letter
            max_word_diff: 5
        }
    }

    pub fn set_max_word_diff(&mut self, diff: u8) {
        self.max_word_diff = diff;
    }

    pub fn insert(&mut self, word: &str) {
        let mut cur_node = &mut self.root;
        for c in word.chars() {
            cur_node = cur_node.children.entry(c)
                                        .or_insert(TrieNode::new(c, false));
        }
        cur_node.is_word = true; // Mark the end of the word
    }

    pub fn find(&self, word: &str) -> bool {
        let mut cur_node = &self.root;
        for c in word.chars() {
            match cur_node.children.get(&c) {
                Some(node) => cur_node = node,
                None => return false, // Character not found
            }
        }
        cur_node.is_word 
    }
}