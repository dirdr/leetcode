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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut digits = vec![];
        let mut curr = head;
        while let Some(node) = curr {
            digits.push(node.val);
            curr = node.next;
        }
        digits.reverse();
        digits.into_iter()
            .enumerate()
            .filter(|(i, bit)| *bit == 1)
            .map(|(i, _)| 2i32.pow(i as u32))
            .sum::<i32>()
    }
}
