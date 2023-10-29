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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut answer: Vec<i32> = vec![];
        Self::dfs(&root, &mut answer);
        answer
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) -> () {
        match root {
            Some(r) => {
                let mut left = Self::dfs(&r.borrow().left, answer);
                answer.push(r.borrow().val);
                let mut right = Self::dfs(&r.borrow().right, answer);
            },
            None => return,
        }
    }
}
