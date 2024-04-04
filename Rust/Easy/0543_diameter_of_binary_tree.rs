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
use std::cmp;
use std::rc::Rc;
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                Some(r) => {
                    let (ld, lh) = dfs(&r.borrow().left);
                    let (rd, rh) = dfs(&r.borrow().right);
                    (ld.max(rd).max(lh + rh), cmp::max(lh, rh) + 1)
                }
                None => (0, 0),
            }
        }
        dfs(&root).0
    }
}
