use std::collections::HashMap;

struct TrieNode { 
    letter: char,
    is_word: bool,
    children: HashMap<char, TrieNode>
}
impl TrieNode{
    fn new(letter: char, is_word: bool) -> Self {
        TrieNode {
            letter,
            is_word,
            children: HashMap::new(),
        }
    }
}

pub struct Trie {
    root: TrieNode,
}
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::new('\0', false), // Root node with no letter
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut cur_node = &mut self.root;
        for c in word.chars() {
            cur_node = cur_node.children.entry(c)
                                        .or_insert(TrieNode::new(c, false));
        }
        cur_node.is_word = true; // Mark the end of the word
    }
}