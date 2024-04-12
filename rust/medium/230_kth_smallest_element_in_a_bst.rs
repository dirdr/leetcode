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
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, count: &mut i32, value: &mut i32) {
            let root = match root {
                Some(root) => root,
                None => return,
            };
            let left = dfs(&root.borrow().left, k, count, value);
            *count -= 1;
            if *count == 0 {
                *value = root.borrow().val;
            }
            let right = dfs(&root.borrow().right, k, count, value);
        }
        let mut answer = 0;
        let mut count = k;
        dfs(&root, k, &mut count, &mut answer);
        answer
    }
}
