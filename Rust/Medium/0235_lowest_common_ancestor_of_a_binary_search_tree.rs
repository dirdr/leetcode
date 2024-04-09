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
use std::cmp::Ordering;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let curr = node.borrow().val;
                if (curr >= p_val && curr <= q_val) || (curr >= q_val && curr <= p_val) {
                    return Some(node.clone());
                }
                if curr > p_val && curr > q_val {
                    dfs(&node.borrow().left, p_val, q_val)
                } else {
                    dfs(&node.borrow().right, p_val, q_val)
                } 
            } else {
                None
            }
        }
        dfs(&root, p.as_ref().unwrap().borrow().val, q.as_ref().unwrap().borrow().val)
    }
}
