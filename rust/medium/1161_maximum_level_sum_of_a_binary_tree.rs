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
use std::collections::{VecDeque, HashMap};

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back((root, 1));
        let mut level_sums = HashMap::new();
        while let Some((node, level)) = queue.pop_front() {
            match node {
                None => continue,
                Some(node) => {
                    let node = node.borrow();
                    *level_sums.entry(level).or_insert(0) += node.val;
                    queue.push_back((node.left.clone(), level + 1));
                    queue.push_back((node.right.clone(), level + 1));
                }
            }
        }
        level_sums.into_iter().max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0))).unwrap().0
    }
}
