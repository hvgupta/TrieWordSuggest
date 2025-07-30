use super::Trie;
use crate::trie::key_dist::get_keywords_at_dist;
use crate::trie::node::TrieNode;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

impl Trie {
    pub fn suggest(&self, word: &str) -> Vec<String> {
        let mut top_queue = PriorityQueue::<String, Reverse<u8>>::new();
        let mut dfs = Vec::<(&TrieNode, String, usize, u8)>::new();
        dfs.push((&self.root, String::new(), 0, 0));

        while !dfs.is_empty() {
            let (node, current_word, index, dist) = dfs.pop().unwrap();
            let index_char = match word.chars().nth(index) {
                Some(c) => c,
                None => continue, // If index is out of bounds, skip
            };
            let matched_child = node.children.get(&index_char);
            if let Some(child) = matched_child {
                dfs.push((child, current_word.clone() + &index_char.to_string(), index + 1, dist));
            }

            if dist + 1 > self.max_word_diff {
                continue; // Skip if distance exceeds max allowed
            }
            

        }

        Vec::new() // Placeholder for the actual suggestions
    }
}
