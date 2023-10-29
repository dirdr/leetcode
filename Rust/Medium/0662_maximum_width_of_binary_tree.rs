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
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let root = match root {
            None => return 0,
            Some(r) => r,
        };
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        let mut result = 0;
        queue.push_back((root, 0));
        while !queue.is_empty() {
            let mut next: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
            let mut left = queue.front().unwrap().1;
            let mut right = queue.back().unwrap().1;
            while let Some((node, index)) = queue.pop_front() {
                let node = node.borrow();
                if let Some(ln) = node.left.clone() {
                    next.push_back((ln, 2 * index));
                }
                if let Some(rn) = node.right.clone() {
                    next.push_back((rn, 2 * index + 1));
                }
            }
            result = std::cmp::max(result, (right - left) + 1);
            std::mem::swap(&mut queue, &mut next);
        }
        result
    }
}
