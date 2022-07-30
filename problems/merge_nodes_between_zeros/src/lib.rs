type ListNode = common::linked_list::ListNode<i32>;
pub struct Solution {}

/////////////////////////////////////////////////////

impl Solution {
    pub fn merge_nodes(mut input_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        input_head = input_head.unwrap().next;

        let mut current_sum = 0;

        let mut result_head: Option<Box<ListNode>> = None;
        let mut result_tail: &mut Option<Box<ListNode>> = &mut result_head;

        loop {
            if let Some(node) = input_head {
                let next_node = node.next;
                match node.val {
                    0 => {
                        if let Some(res_tail) = result_tail {
                            res_tail.next = Some(Box::new(ListNode::new(current_sum)));
                            result_tail = &mut res_tail.next;
                        } else {
                            result_head = Some(Box::new(ListNode::new(current_sum)));
                            result_tail = &mut result_head;
                        }
                        current_sum = 0;
                    }
                    val => {
                        current_sum += val;
                    }
                }
                input_head = next_node;
            } else {
                break;
            }
        }

        result_head
    }
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::Solution;
    use common::linked_list::{list_to_vec, vec_to_list};
    use rstest::rstest;

    #[rstest]
    #[case(vec![0,3,1,0,4,5,2,0],vec![4,11])]
    #[case(vec![0,1,0,3,0,2,2,0], vec![1,3,4])]
    fn simple_tests(#[case] input: Vec<i32>, #[case] expected: Vec<i32>) {
        let input_list = vec_to_list(input);
        let output_list = Solution::merge_nodes(input_list);
        let output = list_to_vec(output_list);

        assert_eq!(output, expected);
    }
}
