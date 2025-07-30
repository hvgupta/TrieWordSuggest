use super::Trie;
use crate::trie::key_dist::get_keywords_at_dist;
use crate::trie::node::TrieNode;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;



impl Trie {
    pub fn suggest(&self, word: &str) -> Vec<String> {
        let mut top_queue = PriorityQueue::<String, Reverse<u8>>::new();
        let mut dfs = Vec::<(&TrieNode, String, usize, u8)>::new();

        dfs.push((&self.root, String::new(), 0, word.len() as u8));

        while !dfs.is_empty() {
            let (node, current_word, index, dist) = dfs.pop().unwrap();
            let index_char = match word.chars().nth(index) {
                Some(c) => c,
                None => continue, // If index is out of bounds, skip
            };
            let matched_child = node.children.get(&index_char);
            if let Some(child) = matched_child {
                let next_word = current_word.clone() + &index_char.to_string();
                dfs.push((
                    child,
                    next_word.clone(),
                    index + 1,
                    dist - 1,
                ));
            }

            if dist - (word.len() as u8) > self.max_word_diff {
                continue; // Skip if distance exceeds max allowed
            }

            if dist < self.max_word_diff {
                top_queue.push(current_word.clone(), Reverse(dist));
            }
            
            let close_keys = get_keywords_at_dist(&index_char, &2);
            for key in close_keys {
                let matched_child = match node.children.get(&key) {
                    Some(child) => child,
                    None => continue, // If no child matches, skip
                };
                dfs.push((
                    matched_child,
                    current_word.clone() + &key.to_string(),
                    index + 1,
                    dist + 1,
                ));
            }
        }

        let mut suggested_words = Vec::new(); // Placeholder for the actual suggestions
        let (word, smallest_dist) = match top_queue.pop() {
            Some((word, Reverse(dist))) => (word, dist),
            None => return suggested_words, 
        };

        suggested_words.push(word);

        while let Some((word, Reverse(_smallest_dist))) = top_queue.pop() {
            if _smallest_dist != smallest_dist {
                break; // Stop if we reach a word with a different distance
            }
            suggested_words.push(word);
        }

        suggested_words
    }
}
