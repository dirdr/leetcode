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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::dfs(&root, target_sum)
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(r) => {
                let r = r.borrow();
                let left = Self::dfs(&r.left, target_sum - r.val);
                let right = Self::dfs(&r.right, target_sum  - r.val);
                if r.left.clone().is_none() && r.right.clone().is_none() {
                    r.val == target_sum
                } else {
                    left || right
                }
            }
        }
    }
}
