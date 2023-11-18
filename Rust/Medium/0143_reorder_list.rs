// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::VecDeque;

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut deque: VecDeque<Box<ListNode>> = VecDeque::new();
        // because we don't wan't to change head
        let mut curr = head.as_mut().unwrap().next.take();
        while let Some(mut node) = curr {
            curr = node.next.take();
            deque.push_back(node);
        }
        let mut count = 0;
        let mut curr = head.as_mut().unwrap();
        while !deque.is_empty() {
            if let Some(node) = deque.pop_back() {
                curr.next = Some(node);
                curr = curr.next.as_mut().unwrap();
            }
            if let Some(node) = deque.pop_front() {
                curr.next = Some(node);
                curr = curr.next.as_mut().unwrap();
            }
        }
    }
}
