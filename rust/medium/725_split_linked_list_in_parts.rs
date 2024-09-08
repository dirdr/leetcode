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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut curr = head.as_ref();
        let mut len = 0;
        while let Some(node) = curr {
            len += 1;
            curr = node.next.as_ref();
        }
        let base_size = len / k as usize;
        let mut extra = len % k as usize;
        let mut result = Vec::with_capacity(k as usize);
        let mut current = head;
        for _ in 0..k {
            let mut part_size = base_size;
            if extra > 0 {
                part_size += 1;
                extra -= 1;
            }

            let mut part_head = None;
            let mut part_tail = &mut part_head;

            for _ in 0..part_size {
                if let Some(mut node) = current.take() {
                    current = node.next.take();
                    *part_tail = Some(node);
                    part_tail = &mut part_tail.as_mut().unwrap().next;
                }
            }
            result.push(part_head);
        }
        result
    }
}
