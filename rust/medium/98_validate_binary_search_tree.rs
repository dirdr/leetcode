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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // A valid BST have ALL the nodes of the left lower than the current one,
        // and ALL of the nodes on the right higher than the current one,
        // so we keep track of the bound we need to be in the function parameters
        // going left, we set the max bound, and going right we set the low bound
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
            let root = match root {
                Some(r) => r,
                None => return true,
            };
            let val = root.borrow().val;
            if val as i64 >= upper || val as i64 <= lower {
                return false;
            }
            dfs(&root.borrow().left, lower, val as i64) && dfs(&root.borrow().right, val as i64, upper)
        }
        dfs(&root, i32::MIN as i64 - 1, i32::MAX as i64 + 1)
    }
}
