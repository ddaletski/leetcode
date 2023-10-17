use crate::common::trie::TrieNode;
type Trie = crate::common::trie::Trie<String>;

pub struct Solution {}

/////////////////////////////////////////////////////
impl Solution {
    pub fn unique_forward_paths(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut trie = Trie::new();
        for path in paths {
            trie.insert(path.into_iter());
        }

        let mut result = vec![];

        let mut stack = vec![];
        stack.push(trie.root());

        while let Some(node) = stack.pop() {
            if node.children_count() == 0 {

            }
        }

        result
    }

    fn rec_fn<'a>(node: &'a TrieNode<String>, current_path: &mut Vec<&'a TrieNode<String>>) {
        current_path.push(node);

        match node.children_count() {
            0 => {}
            1 => {
                Solution::rec_fn(node.children().next().unwrap().1, current_path);
            }
            _ => {
                println!("current path has multiple children");
                println!(
                    "{:?}",
                    current_path
                        .iter()
                        .map(|node| &node.character)
                        .collect::<Vec<_>>()
                );
                // mark
                for (_, child_node) in node.children() {
                    Solution::rec_fn(child_node, current_path);
                }
            }
        }

        current_path.pop();
    }

    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut paths_trie = Trie::new();
        for path in paths {
            paths_trie.insert(path.into_iter().rev());
        }

        let mut current_path = vec![];
        Solution::rec_fn(paths_trie.root(), &mut current_path);

        println!("{:?}", paths_trie);

        vec![]
    }
}
//////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{assert_eq, vec2d};
    use rstest::rstest;

    #[rstest]
    #[case(
        vec2d![["a"],["c"],["a","b"],["c","b"],["a","b","x"],["a","b","x","y"],["w"],["w","y"]],
        vec2d![["c"],["c","b"],["a"],["a","b"]]
    )]
    fn it_works(#[case] _input: Vec<Vec<&str>>, #[case] _expected_result: Vec<Vec<&str>>) {
        let input: Vec<Vec<String>> = _input
            .into_iter()
            .map(|strs| strs.into_iter().map(|s| s.to_owned()).collect())
            .collect();

        let expected_result: Vec<Vec<String>> = _expected_result
            .into_iter()
            .map(|strs| strs.into_iter().map(|s| s.to_owned()).collect())
            .collect();

        let actual_result = Solution::delete_duplicate_folder(input);

        assert_eq!(actual_result, expected_result);
    }
}
