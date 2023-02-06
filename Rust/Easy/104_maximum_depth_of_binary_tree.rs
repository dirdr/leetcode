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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dfs(&root)    
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(r) => {
                let left = Self::dfs(&r.borrow().left);
                let right = Self::dfs(&r.borrow().right);
                return std::cmp::max(left, right) + 1;
            },
            None => return 0,
        }
    }
}
