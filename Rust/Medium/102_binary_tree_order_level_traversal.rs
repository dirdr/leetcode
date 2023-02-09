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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let root = match root {
            None => return vec![],
            Some(r) => r
        };
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut result = vec![];
        queue.push_back(root);
        while (!queue.is_empty()) {
            let mut next: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            let mut temp_container = vec![];
            while let Some(node) = queue.pop_front() {
                let node = node.borrow();
                temp_container.push(node.val);
                if let Some(ln) = node.left.clone() {
                    next.push_back(ln);
                }
                if let Some(rn) = node.right.clone() {
                    next.push_back(rn);
                }
            }
            result.push(temp_container);
            std::mem::swap(&mut queue, &mut next);
        }
        result
    }
}
