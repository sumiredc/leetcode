mod list_node;

use list_node::ListNode;

/// @see https://leetcode.com/problems/merge-two-sorted-lists/description/
///
/// You are given the heads of two sorted linked lists list1 and list2.
/// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
/// Return the head of the merged linked list.
///
/// Constraints:
/// - The number of nodes in both lists is in the range [0, 50].
/// - -100 <= Node.val <= 100
/// - Both list1 and list2 are sorted in non-decreasing order.
///
/// @time_complexity  O(N)
/// @space_complexity O(1)
///
impl Solution {
    // case 1
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(ln), None) => Some(ln),
            (None, Some(ln)) => Some(ln),
            (Some(ln1), Some(ln2)) => match ln1.val <= ln2.val {
                true => Some(Box::new(ListNode {
                    val: ln1.val,
                    next: Self::merge_two_lists(ln1.next, Some(ln2)),
                })),
                false => Some(Box::new(ListNode {
                    val: ln2.val,
                    next: Self::merge_two_lists(Some(ln1), ln2.next),
                })),
            },
        }
    }

    // case2
    pub fn merge_two_lists2(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::mem::swap;
        let mut head = None;
        let mut tail = &mut head;
        loop {
            if list1.is_none() {
                swap(tail, &mut list2);
                return head;
            }
            if list2.is_none() {
                swap(tail, &mut list1);
                return head;
            }
            if list1.as_ref().unwrap().val < list2.as_ref().unwrap().val {
                swap(tail, &mut list1);
                tail = &mut tail.as_mut().unwrap().next;
                swap(tail, &mut list1);
            } else {
                swap(tail, &mut list2);
                tail = &mut tail.as_mut().unwrap().next;
                swap(tail, &mut list2);
            }
        }
    }

    // case3
    pub fn merge_two_lists3(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }
        let mut head = ListNode::new(-1);
        let mut tail = &mut head;
        let mut list1 = list1;
        let mut list2 = list2;

        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                tail.next = list1.take();
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2.take();
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        if list1.is_some() {
            tail.next = list1.take();
        }
        if list2.is_some() {
            tail.next = list2.take();
        }
        head.next
    }
}
pub struct Solution;

use rstest::rstest;

#[rstest]
#[case(vec![1,2,4], vec![1,3,4], vec![1,1,2,3,4,4])]
#[case(vec![], vec![], vec![])]
#[case(vec![], vec![0], vec![0])]
fn merge_two_lists_test(
    #[case] list1: Vec<i32>,
    #[case] list2: Vec<i32>,
    #[case] expected: Vec<i32>,
) {
    let actual = Solution::merge_two_lists(ListNode::extend(list1), ListNode::extend(list2));
    assert_eq!(actual, ListNode::extend(expected));
}

fn main() {}
