use super::Trie;
use crate::trie::key_dist::get_keywords_at_dist;
use crate::trie::node::TrieNode;

impl Trie {
    pub fn suggest(&self, word: &str) -> Vec<String> {
        let dfs = Vec::<(&TrieNode, String, i8)>::new();
        dfs.push((&self.root, String::new(), -1));
    }
}
