use std::collections::VecDeque;
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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let Some(root) = root else { return vec![] };
        let mut queue = VecDeque::new();
        queue.push_front(root);
        let mut answer = vec![];
        while !queue.is_empty() {
            let mut next = VecDeque::new();
            let mut max = i32::MIN;
            while let Some(node) = queue.pop_back() {
                let node = node.borrow();
                max = max.max(node.val);
                if let Some(ln) = node.left.clone() {
                    next.push_front(ln);
                }
                if let Some(rn) = node.right.clone() {
                    next.push_front(rn);
                }
            }
            answer.push(max);
            std::mem::swap(&mut queue, &mut next);
        }
        answer
    }
}
