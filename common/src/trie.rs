use std::{collections::HashMap, fmt::Debug, str::Chars};

pub struct TrieNode {
    pub character: char,
    pub word_end: bool,
    pub children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    pub fn new(character: char, word_end: bool) -> Self {
        TrieNode {
            character,
            word_end,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str) -> bool {
        self.insert_impl(word.chars())
    }

    pub fn contains(&self, word: &str) -> bool {
        if let Some(node) = self.find_impl(word.chars()) {
            node.word_end
        } else {
            false
        }
    }

    pub fn find_prefix(&self, prefix: &str) -> Option<&Self> {
        self.find_impl(prefix.chars())
    }

    pub fn next(&self, next_char: char) -> Option<&Self> {
        self.children.get(&next_char).map(|b| b.as_ref())
    }

    fn insert_impl(&mut self, mut word: Chars) -> bool {
        if let Some(next_char) = word.next() {
            match &mut self.children.get_mut(&next_char) {
                Some(next_node) => {
                    return next_node.insert_impl(word);
                }
                None => {
                    let mut next_node = Box::new(TrieNode::new(next_char, false));
                    let result = next_node.insert_impl(word);

                    self.children.insert(next_char, next_node);
                    return result;
                }
            }
        }

        self.word_end = true;
        true
    }

    fn find_impl(&self, mut word: Chars) -> Option<&Self> {
        if let Some(next_char) = word.next() {
            if let Some(next_node) = &self.children.get(&next_char) {
                return next_node.find_impl(word);
            } else {
                return None;
            }
        }
        Some(self)
    }

    fn fill_all_children(&self, current_word: &mut Vec<char>, result: &mut Vec<String>) {
        if self.word_end {
            result.push(current_word.iter().collect());
        }

        self.children.iter().for_each(|(&next_char, next_node)| {
            current_word.push(next_char);
            next_node.fill_all_children(current_word, result);
            current_word.pop();
        });
    }

    fn format_impl(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space = "| ".repeat(indent);
        f.write_fmt(format_args!("{}{}\n", space, self.character))?;

        for node in self.children.values() {
            node.format_impl(indent + 1, f)?;
        }

        Ok(())
    }
}

impl Debug for TrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format_impl(0, f)?;
        Ok(())
    }
}

pub struct Trie {
    root: Box<TrieNode>,
    words_count: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Box::new(TrieNode::new('@', false)),
            words_count: 0,
        }
    }

    pub fn insert(&mut self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        let inserted = self.root.insert(word);

        if inserted {
            self.words_count += 1;
        }

        inserted
    }

    pub fn contains(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        self.root.contains(word)
    }

    pub fn root(&self) -> &TrieNode {
        &self.root
    }

    pub fn find_prefix(&self, prefix: &str) -> Option<&TrieNode> {
        self.root.find_prefix(prefix)
    }

    pub fn find_all(&self, prefix: &str) -> Vec<String> {
        let mut result = vec![];

        if let Some(starting_point) = self.root.find_prefix(prefix) {
            let mut current_word = prefix.to_owned().chars().collect();
            starting_point.fill_all_children(&mut current_word, &mut result);
        }

        result
    }

    pub fn len(&self) -> usize {
        self.words_count
    }

    pub fn is_empty(&self) -> bool {
        self.words_count == 0
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.root.fmt(f)?;
        Ok(())
    }
}

impl From<Vec<&str>> for Trie {
    fn from(value: Vec<&str>) -> Self {
        let mut trie = Trie::new();
        value.into_iter().for_each(|s| {
            trie.insert(s);
        });

        trie
    }
}

#[cfg(test)]
mod test {
    use super::Trie;
    use crate::assert_returns;
    use rstest::{fixture, rstest};
    use std::path::PathBuf;

    use lazy_static::lazy_static;
    use proptest::proptest;
    use rand::{
        self,
        distributions::Alphanumeric,
        distributions::{DistString, Uniform},
        prelude::Distribution,
    };

    lazy_static! {
        static ref RAND_WORDS: Vec<String> = {
            let mut rng = rand::thread_rng();
            let len_distribution = Uniform::<usize>::from(1..10);

            let mut random_words: Vec<String> = vec![];

            for _ in 0..=100 {
                let word_len = len_distribution.sample(&mut rng);
                let word = Alphanumeric.sample_string(&mut rng, word_len);
                random_words.push(word);
            }

            random_words
        };
        static ref WORDS_100: Vec<String> = {
            let words_file_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/words100.txt");

            std::fs::read_to_string(words_file_path)
                .unwrap()
                .trim()
                .split("\n")
                .map(|s| s.to_owned())
                .collect()
        };
    }

    #[fixture]
    fn top100words() -> Vec<&'static str> {
        WORDS_100.iter().map(|s| s.as_str()).collect()
    }

    #[rstest]
    fn top100words_has_100_words(top100words: Vec<&str>) {
        assert_eq!(top100words.len(), 100);
    }

    #[fixture]
    fn random_words() -> Vec<&'static str> {
        RAND_WORDS.iter().map(|s| s.as_str()).collect()
    }

    #[fixture]
    fn lol_kek_chebureck_list() -> Vec<&'static str> {
        vec!["lol", "kek", "chebureck"]
    }

    #[fixture]
    fn top100trie(top100words: Vec<&str>) -> Trie {
        Trie::from(top100words)
    }

    #[fixture]
    fn random_trie(random_words: Vec<&str>) -> Trie {
        Trie::from(random_words)
    }

    #[fixture]
    fn lol_kek_chebureck_trie(lol_kek_chebureck_list: Vec<&str>) -> Trie {
        Trie::from(lol_kek_chebureck_list)
    }

    mod trie_contains_inserted_words {
        use super::*;

        #[rstest]
        fn random(random_trie: Trie, random_words: Vec<&str>) {
            for word in random_words {
                assert_returns!(true, Trie::contains, &random_trie, word);
            }
        }

        #[rstest]
        fn top100(top100trie: Trie, top100words: Vec<&str>) {
            for word in top100words {
                assert_returns!(true, Trie::contains, &top100trie, word);
            }
        }
    }

    mod trie_finds_inserted_words {
        use super::*;

        #[rstest]
        fn random(random_trie: Trie, random_words: Vec<&str>) {
            for word in random_words {
                let returned_node = random_trie.find_prefix(word);
                assert!(returned_node.is_some());
                assert!(returned_node.unwrap().word_end);
            }
        }

        #[rstest]
        fn top100(top100trie: Trie, top100words: Vec<&str>) {
            for word in top100words {
                let returned_node = top100trie.find_prefix(word);
                assert!(returned_node.is_some());
                assert!(returned_node.unwrap().word_end);
            }
        }
    }

    #[rstest]
    fn find_incomplete_word_returns_node_with_word_end_equals_false(
        lol_kek_chebureck_trie: Trie,
        lol_kek_chebureck_list: Vec<&str>,
    ) {
        for word in lol_kek_chebureck_list {
            let incomplete_word = &word[0..word.len() - 1];
            let found_node = lol_kek_chebureck_trie.find_prefix(incomplete_word);

            assert!(found_node.is_some());
            assert!(!found_node.unwrap().word_end);
        }
    }

    mod find_all {
        use super::*;

        #[rstest]
        fn empty_prefix_populates_all_words(
            lol_kek_chebureck_trie: Trie,
            lol_kek_chebureck_list: Vec<&str>,
        ) {
            let mut expected_result: Vec<String> = lol_kek_chebureck_list
                .into_iter()
                .map(String::from)
                .collect();
            expected_result.sort();

            let mut result = lol_kek_chebureck_trie.find_all("");
            result.sort();

            assert_eq!(result, expected_result);
        }

        #[rstest]
        fn find_all_works() {
            let words_list = vec![
                "abcaaaa", "abdaa", "bca", "abc0010", "abc", "0abc", "abcabc",
            ];
            let prefix = "abc";
            let mut expected_result = vec!["abcaaaa", "abc0010", "abc", "abcabc"];
            expected_result.sort();

            let trie = Trie::from(words_list);

            let mut result = trie.find_all(prefix);
            result.sort();

            assert_eq!(result, expected_result);
        }
    }

    mod trie_size_is_correct {
        use super::*;

        #[rstest]
        fn empty_has_zero_len() {
            assert_returns!(0, Trie::len, &Trie::new());
        }

        #[rstest]
        fn top100trie_has_len_of_100(top100trie: Trie) {
            assert_returns!(100, Trie::len, &top100trie);
        }
    }

    proptest! {
        #[test]
        fn empty_trie_contains_nothing(ref word in ".*") {
            let empty_trie = Trie::new();

            assert!(!empty_trie.contains(word))
        }
    }
}
