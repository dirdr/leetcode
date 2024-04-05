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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
            match root {
                Some(node) => {
                    let (left, bl) = dfs(&node.borrow().left);
                    let (right, br) = dfs(&node.borrow().right);
                    (cmp::max(left, right) + 1, (left - right).abs() <= 1 && bl && br) 
                },
                None => (0, true)
            }
        }
        dfs(&root).1
    }
}
