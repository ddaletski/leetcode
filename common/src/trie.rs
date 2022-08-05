use std::{fmt::Debug, mem::MaybeUninit, str::Chars};

const A_Z_RANGE_SIZE: u8 = 'z' as u8 - 'a' as u8;
const ALPHABET_SIZE: u8 = 2 * A_Z_RANGE_SIZE + 10;

pub struct TrieNode {
    pub character: char,
    pub word_end: bool,
    pub children: [Option<Box<TrieNode>>; ALPHABET_SIZE as usize],
}

impl TrieNode {
    pub fn new(character: char, word_end: bool) -> Self {
        TrieNode {
            character,
            word_end,
            children: {
                let mut arr_data: [MaybeUninit<Option<Box<TrieNode>>>; ALPHABET_SIZE as usize] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for item in &mut arr_data[..] {
                    item.write(None);
                }

                unsafe { std::mem::transmute(arr_data) }
            },
        }
    }

    pub fn insert(&mut self, word: &str) -> bool {
        self.insert_impl(word.chars())
    }

    pub fn contains(&self, word: &str) -> bool {
        self.contains_impl(word.chars())
    }

    fn insert_impl(&mut self, mut word: Chars) -> bool {
        if let Some(next_char) = word.next() {
            let code = ord(next_char) as usize;

            match &mut self.children[code] {
                Some(next_node) => {
                    return next_node.insert_impl(word);
                }
                None => {
                    let mut next_node = Box::new(TrieNode::new(next_char, false));
                    let result = next_node.insert_impl(word);

                    self.children[code] = Some(next_node);
                    return result;
                }
            }
        }

        self.word_end = true;
        return true;
    }

    fn contains_impl(&self, mut word: Chars) -> bool {
        if let Some(next_char) = word.next() {
            let code = ord(next_char) as usize;

            return if let Some(next_node) = &self.children[code] {
                next_node.contains_impl(word)
            } else {
                false
            };
        } else {
            self.word_end
        }
    }

    fn format_impl(&self, indent: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space = "| ".repeat(indent);
        f.write_fmt(format_args!("{}{}\n", space, self.character))?;

        for node in self.children.iter().filter_map(|child| child.as_ref()) {
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
    root: Option<Box<TrieNode>>,
    words_count: usize,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: None,
            words_count: 0,
        }
    }

    pub fn insert(&mut self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        let inserted = if let Some(root_node) = &mut self.root {
            root_node.insert(word)
        } else {
            let mut root_node = Box::new(TrieNode::new('@', false));
            root_node.insert(word);
            self.root = Some(root_node);

            !word.is_empty()
        };

        if inserted {
            self.words_count += 1;
        }

        inserted
    }

    pub fn contains(&self, word: &str) -> bool {
        if word.is_empty() {
            return false;
        }

        if let Some(root_node) = &self.root {
            root_node.contains(word)
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.words_count
    }
}

impl Debug for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root) = &self.root {
            root.fmt(f)?;
        } else {
            f.write_str("<empty>")?;
        }
        Ok(())
    }
}

pub fn ord(ch: char) -> u8 {
    match ch {
        'a'..='z' => ch as u8 - 'a' as u8,
        'A'..='Z' => (ch as u8 - 'A' as u8) + A_Z_RANGE_SIZE,
        '0'..='9' => (ch as u8 - '0' as u8) + 2 * A_Z_RANGE_SIZE,
        _ => panic!("unsupported character {}", ch),
    }
}

pub fn chr(code: u8) -> char {
    if code < A_Z_RANGE_SIZE {
        (code + 'a' as u8) as char
    } else if code < 2 * A_Z_RANGE_SIZE {
        (code - A_Z_RANGE_SIZE + 'A' as u8) as char
    } else if code < ALPHABET_SIZE {
        (code - 2 * A_Z_RANGE_SIZE + '0' as u8) as char
    } else {
        panic!("unsupported character code {}", code)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lazy_static::lazy_static;
    use proptest::proptest;
    use rand::{
        self,
        distributions::Alphanumeric,
        distributions::{DistString, Uniform},
        prelude::Distribution,
    };
    use rstest::{fixture, rstest};

    use super::Trie;

    macro_rules! assert_returns {
        // This macro takes an expression of type `expr` and prints
        // it as a string along with its result.
        // The `expr` designator is used for expressions.
        ($ret_value:expr, $func:expr, $($args:expr),*) => {
            // `stringify!` will convert the expression *as it is* into a string.
            let mut error_msg = format!(
                "expected result: {:?},\nfunction: {:?},\nargs:",
                $ret_value,
                stringify!($func),
                );
            $(
                error_msg += format!("\n  {:?}", $args).as_str();
            )*
                error_msg += "\n";

            assert_eq!($func($($args),*), $ret_value, "\n{:}", error_msg);
        };
    }

    #[fixture]
    fn empty_trie() -> Trie {
        Trie::new()
    }

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
    fn top100trie(mut empty_trie: Trie, top100words: Vec<&str>) -> Trie {
        for word in top100words.into_iter() {
            empty_trie.insert(&word);
        }

        empty_trie
    }

    #[fixture]
    fn random_trie(mut empty_trie: Trie, random_words: Vec<&str>) -> Trie {
        for word in random_words.into_iter() {
            empty_trie.insert(&word);
        }

        empty_trie
    }

    mod trie_contains_inserted_items {
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

    mod trie_size_is_correct {
        use super::*;

        #[rstest]
        fn empty_has_zero_len(empty_trie: Trie) {
            assert_returns!(0, Trie::len, &empty_trie);
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
