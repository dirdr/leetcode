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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_ref();
        let mut n = 0;
        while let Some(node) = curr {
            curr = node.next.as_ref();
            n += 1;
        }

        if n == 1 {
            return None;
        }

        let mut curr = head.as_mut();
        let target = (n / 2) - 1;
        for _ in 0..target {
            curr = curr.unwrap().next.as_mut();
        }

        if let Some(node) = curr {
            node.next = node.next.as_mut().unwrap().next.take();
        }
        head
    }
}
