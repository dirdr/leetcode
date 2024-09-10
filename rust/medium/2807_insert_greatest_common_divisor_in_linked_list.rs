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
    pub fn insert_greatest_common_divisors(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut curr = &mut head;
        while let Some(node) = curr {
            if let Some(next) = &node.next {
                let gcd_val = Self::gcd(node.val, next.val);
                let new_node = Box::new(ListNode {
                    val: gcd_val,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                curr = &mut node.next.as_mut().unwrap().next;
            } else {
                break;
            }
        }
        head
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }
}
