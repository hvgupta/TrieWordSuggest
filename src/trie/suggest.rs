use super::Trie;
use crate::trie::key_dist::get_keywords_at_dist;
use crate::trie::node::TrieNode;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

impl Trie {
    fn _subs_and_replace<'a>(
        &self,
        dfs: &mut Vec<(&'a TrieNode, String, usize, u8)>,
        current_word: &String,
        dist: &u8,
        index: &usize,
        index_char: char,
        node: &'a TrieNode,
    ) {
        let close_keys = get_keywords_at_dist(&index_char, &2);
        for key in close_keys {
            let matched_child = match node.children.get(&key) {
                Some(child) => child,
                None => continue, // If no child matches, skip
            };
            if key == index_char {
                // If the key matches the current character in the word
                dfs.push((
                    matched_child,
                    current_word.clone() + &key.to_string(),
                    index + 1,
                    dist - 1,
                ));

                continue;
            }
            // If the key is a close match, increase distance
            dfs.push((
                matched_child,
                current_word.clone() + &key.to_string(),
                index + 1,
                dist + 1,
            ));
        }
    }

    fn _insert<'a>(
        &self,
        dfs: &mut Vec<(&'a TrieNode, String, usize, u8)>,
        current_word: &String,
        dist: &u8,
        node: &'a TrieNode,
    ) {
        for key in node.children.keys() {
            if *key == '\0' {
                continue; // Skip the root node character
            }
            let new_word = current_word.clone() + &key.to_string();
            dfs.push((node.children.get(key).unwrap(), new_word, 0, dist + 1));
        }
    }

    pub fn suggest(&self, word: &str, max_dist: u8) -> Vec<String> {
        let mut top_queue = PriorityQueue::<String, Reverse<u8>>::new();
        let mut dfs = Vec::<(&TrieNode, String, usize, u8)>::new();

        let len = word.len();

        dfs.push((&self.root, String::new(), 0, len as u8));

        while let Some((node, current_word, index, dist)) = dfs.pop() {
            println!("Current word: {}, Index: {}, Dist: {}", current_word, index, dist);

            if current_word == word {
                return vec![current_word]; // If the current word matches the input word, return it
            }

            if dist > self.max_word_diff + word.len() as u8 {
                continue; // Skip if distance exceeds max allowed
            }

            if (dist <= self.max_word_diff) && node.is_word {
                top_queue.push(current_word.clone(), Reverse(dist));
            }

            match word.chars().nth(index) {
                Some(index_char) => self._subs_and_replace(
                    &mut dfs,
                    &current_word,
                    &dist,
                    &index,
                    index_char,
                    node,
                ),
                None => self._insert(&mut dfs, &current_word, &dist, node), // If index is out of bounds, skip
            };
        }

        let (word, smallest_dist) = match top_queue.pop() {
            Some((word, Reverse(dist))) => (word, dist),
            None => return vec![], 
        };

        if smallest_dist > max_dist {
            return vec![]; // If the smallest distance is greater than max_dist, return empty
        }

        let mut suggested_words = Vec::new();
        suggested_words.push(word);

        while let Some((word, Reverse(dist))) = top_queue.pop() {
            if dist > max_dist {
                break; // Stop if we reach a word with a different distance
            }
            suggested_words.push(word);
        }

        suggested_words
    }
}
