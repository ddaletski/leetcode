use std::collections::{HashMap, HashSet};

use common::trie::Trie;

struct WordFilter {
    forward_trie: Trie,
    backward_trie: Trie,
    index_mapping: HashMap<String, i32>,
}

impl WordFilter {
    fn new(words: Vec<String>) -> Self {
        let mut fw = Trie::new();
        let mut bw = Trie::new();

        words.iter().for_each(|word| {
            let word_reversed: String = word.chars().rev().collect();
            fw.insert(&word);
            bw.insert(&word_reversed);
        });

        let map: HashMap<String, i32> = words
            .into_iter()
            .enumerate()
            .map(|(idx, word)| (word, idx as i32))
            .collect();

        Self {
            forward_trie: fw,
            backward_trie: bw,
            index_mapping: map,
        }
    }

    fn f(&self, pref: String, suff: String) -> i32 {
        let suffix_reversed: String = suff.chars().rev().collect();

        let prefix_set: HashSet<String> = self.forward_trie.find_all(&pref).into_iter().collect();
        let suffix_set: HashSet<String> = self
            .backward_trie
            .find_all(&suffix_reversed)
            .into_iter()
            .map(|word_rev| word_rev.chars().rev().collect())
            .collect();

        let common_set = prefix_set.intersection(&suffix_set);

        let max_index = common_set
            .map(|word| *self.index_mapping.get(word).unwrap_or(&-1))
            .max()
            .unwrap_or(-1);

        max_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = vec!["lol", "apple", "kek", "ape", "chebureck"]
            .into_iter()
            .map(String::from)
            .collect();
        let filter = WordFilter::new(words);

        assert_eq!(filter.f("a".into(), "e".into()), 3);
    }
}
