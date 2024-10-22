// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::collections::{BinaryHeap, VecDeque};
use std::rc::Rc;

impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut heap: BinaryHeap<i64> = BinaryHeap::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        queue.push_front(root.unwrap());
        while !queue.is_empty() {
            let mut next: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            let mut level_sum = 0;
            while let Some(node) = queue.pop_front() {
                let node = node.borrow();
                level_sum += node.val as i64;
                if let Some(ln) = node.left.clone() {
                    next.push_back(ln);
                }
                if let Some(rn) = node.right.clone() {
                    next.push_back(rn);
                }
            }
            heap.push(level_sum);
            std::mem::swap(&mut queue, &mut next);
        }
        let mut k = k;
        while k > 1 {
            heap.pop();
            k -= 1;
        }
        if let Some(val) = heap.pop() {
            return val;
        }
        -1
    }
}
