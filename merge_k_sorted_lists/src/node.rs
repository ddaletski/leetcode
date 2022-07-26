#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    #[inline]
    pub fn new_link(val: i32, next: Option<Box<ListNode>>) -> Option<Box<Self>> {
        Some(Box::new(ListNode { val, next }))
    }

    pub fn from_values(mut values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;

        while !values.is_empty() {
            let last_val = values.pop().unwrap();
            head = ListNode::new_link(last_val, head)
        }
        head
    }

    pub fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = vec![];

        let mut head = list;
        while head.is_some() {
            result.push(head.as_ref().unwrap().val);
            head = head.unwrap().next;
        }

        result
    }
}
