use super::Trie;
use crate::trie::key_dist::get_keywords_at_dist;
use crate::trie::node::TrieNode;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

impl Trie {
    pub fn suggest(&self, word: &str, max_dist: u8) -> Vec<String> {
        let mut top_queue = PriorityQueue::<String, Reverse<u8>>::new();
        let mut dfs = Vec::<(&TrieNode, String, usize, i8)>::new();

        let len = word.len() as i8;

        dfs.push((&self.root, String::new(), 0, len));

        while !dfs.is_empty() {
            let (node, current_word, index, dist) = dfs.pop().unwrap();
            let index_char = match word.chars().nth(index) {
                Some(c) => c,
                None => continue, // If index is out of bounds, skip
            };
            let matched_child = node.children.get(&index_char);
            if let Some(child) = matched_child {
                let next_word = current_word.clone() + &child.letter.to_string();
                dfs.push((child, next_word.clone(), index + 1, dist - 1));
            }

            if dist > self.max_word_diff + word.len() as i8 || dist < -self.max_word_diff {
                continue; // Skip if distance exceeds max allowed
            }

            if (dist.abs() <= self.max_word_diff) && node.is_word {
                top_queue.push(current_word.clone(), Reverse(dist.abs() as u8));
            }

            let close_keys = get_keywords_at_dist(&index_char, &2);
            for key in close_keys {
                let matched_child = match node.children.get(&key) {
                    Some(child) => child,
                    None => continue, // If no child matches, skip
                };
                dfs.push((
                    matched_child,
                    current_word.clone() + &matched_child.letter.to_string(),
                    index + 1,
                    dist + 1,
                ));
            }
        }

        let mut suggested_words = Vec::new();
        let (word, smallest_dist) = match top_queue.pop() {
            Some((word, Reverse(dist))) => (word, dist),
            None => return suggested_words,
        };
        
        if smallest_dist > max_dist {
            return suggested_words; // If the smallest distance is greater than max_dist, return empty
        }

        suggested_words.push(word);

        while let Some((word, Reverse(dist))) = top_queue.pop() {
            if  dist > max_dist{
                break; // Stop if we reach a word with a different distance
            }
            suggested_words.push(word);
        }

        suggested_words
    }
}
