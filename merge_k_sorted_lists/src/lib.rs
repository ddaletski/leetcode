mod node;
use node::ListNode;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}
impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::with_capacity(lists.len());

        for list in lists {
            heap.push(list)
        }

        let mut head: Option<Box<ListNode>> = None;
        let mut tail = &mut head;

        while !heap.is_empty() {
            let min_list = heap.pop().unwrap();

            if let Some(next_node) = min_list {
                match tail {
                    None => {
                        head = ListNode::new_link(next_node.val, None);
                        tail = &mut head;
                    }
                    Some(node) => {
                        node.next = ListNode::new_link(next_node.val, None);
                        tail = &mut node.next;
                    }
                }
                heap.push(next_node.next)
            }
        }

        head
    }
}

#[cfg(test)]
mod tests {
    use crate::{ListNode, Solution};
    #[test]
    fn parse_dump() {
        let input = vec![1, 2, 3, 4, 5];

        let parsed = ListNode::from_values(input.clone());
        let dumped = ListNode::to_vec(parsed);
        println!("{:?}", dumped);

        assert_eq!(input, dumped);
    }

    #[test]
    fn it_works() {
        let input_values = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]];
        let expected_values = vec![1, 1, 2, 3, 4, 4, 5, 6];

        let input_list: Vec<_> = input_values
            .into_iter()
            .map(|vals| ListNode::from_values(vals))
            .collect();

        let result_list = Solution::merge_k_lists(input_list);

        let result_values = ListNode::to_vec(result_list);

        assert_eq!(result_values, expected_values);
    }

    #[test]
    fn empty_input() {
        let empty_input = vec![None];
        let result = Solution::merge_k_lists(empty_input);

        assert_eq!(result, None);
    }
}
