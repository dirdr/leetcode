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
impl Solution {
    pub fn helper(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        remainder: i32,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => {
                if remainder > 0 {
                    Some(Box::new(ListNode::new(remainder)))
                } else {
                    None
                }
            }
            (n1, n2) => {
                let new_value = n1.as_ref().map_or(0, |node| node.val)
                    + n2.as_ref().map_or(0, |node| node.val)
                    + remainder;
                let mut node = Some(Box::new(ListNode::new(new_value % 10)));
                node.as_mut().unwrap().next = Self::helper(
                    n1.and_then(|node| node.next),
                    n2.and_then(|node| node.next),
                    new_value / 10,
                );
                node
            }
        }
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::helper(l1, l2, 0)
    }
}
