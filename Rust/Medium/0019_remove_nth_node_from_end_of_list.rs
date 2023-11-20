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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // two pass solution
        let mut dummy = Some(Box::new(ListNode::new(-1)));
        dummy.as_mut().unwrap().next = head;
        let mut count = 0;
        let mut curr = dummy.as_ref();
        // while the next reference is still a node
        while let Some(node) = curr.unwrap().next.as_ref() {
            count += 1;
            curr = Some(node);
        }
        let mut curr = dummy.as_mut();
        for _ in 1..(count - n + 1) {
            curr = curr.unwrap().next.as_mut();
        }
        // remove nth from end
        // without the take, the value of the next-next is still owned by the option,
        // we need to 'take' the value out of the option
        curr.unwrap().next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
        dummy.unwrap().next.take()
    }
}
