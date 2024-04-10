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
use std::cmp;
impl Solution {
    // preorder traversal, keeping track of the maximum value encountered
    // in the current path.
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
            let root = match root {
                Some(r) => r,
                None => return 0,
            };
            let good = if root.borrow().val >= max {1} else {0};
            let max = cmp::max(root.borrow().val, max);
            dfs(&root.borrow().left, max) +
            dfs(&root.borrow().right, max) +
            good
        }
        dfs(&root, i32::MIN)
    }
}
