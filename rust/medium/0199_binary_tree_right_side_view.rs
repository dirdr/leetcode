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
use std::collections::VecDeque;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let root = match root {
            None => return vec![],
            Some(r) => r,
        };
        let mut queue = VecDeque::new();
        queue.push_back(root);
        let mut result = vec![];
        while (!queue.is_empty()) {
            let mut container = vec![];
            let mut next = VecDeque::new();
            while let Some(node) = queue.pop_front() {
                let node = node.borrow();
                container.push(node.val);
                if let Some(ln) = node.left.clone() {
                    next.push_back(ln);
                }
                if let Some(rn) = node.right.clone() {
                    next.push_back(rn);
                }
            }
            result.push(container.last().cloned().unwrap());
            container.clear();
            std::mem::swap(&mut queue, &mut next);
        }
        result
    }
}
