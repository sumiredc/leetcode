// Definition for singly-linked list.
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

    pub fn extend(nums: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in nums.iter().rev() {
            let new_node = ListNode {
                val: val,
                next: head.take(),
            };
            head = Some(Box::new(new_node));
        }
        head
    }
}
