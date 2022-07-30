#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    fn new_link(val: T, next: Option<Box<Self>>) -> Option<Box<Self>> {
        Some(Box::new(ListNode { val, next }))
    }
}

pub fn vec_to_list<T>(mut values: Vec<T>) -> Option<Box<ListNode<T>>> {
    let mut head = None;

    while !values.is_empty() {
        let last_val = values.pop().unwrap();
        head = ListNode::new_link(last_val, head)
    }
    head
}

pub fn list_to_vec<T>(list: Option<Box<ListNode<T>>>) -> Vec<T> {
    let mut result = vec![];

    let mut head = list;
    loop {
        if let Some(node) = head {
            result.push(node.val);
            head = node.next;
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::linked_list::{vec_to_list, ListNode};
    #[test]
    fn parse_dump() {
        let input = vec![1, 2, 3, 4, 5];

        let parsed = vec_to_list::<i32>(input.clone());
        let dumped = list_to_vec(parsed);

        assert_eq!(input, dumped);
    }
}
