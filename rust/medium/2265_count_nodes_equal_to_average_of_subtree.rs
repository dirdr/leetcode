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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            match root {
                None => (0, 0, 0),
                Some(node) => {
                    let node = node.borrow();
                    let (ls, ld, lc) = dfs(&node.left);
                    let (rs, rd, rc) = dfs(&node.right);
                    let average = (ls + rs + node.val) / (ld + rd + 1);
                    let bonus = if node.val == average { 1 } else { 0 };
                    (ls + rs + node.val, ld + rd + 1, lc + rc + bonus)
                }
            }
        }
        dfs(&root).2
    }
}
