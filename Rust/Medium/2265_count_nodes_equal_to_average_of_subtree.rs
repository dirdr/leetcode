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

struct Container {
    size: i32,
    sum: i32
}

impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut answer = 0;
        Self::dfs(&root, &mut answer);
        answer
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut i32) -> Container {
        match root {
            Some(root) => {
                let root = root.borrow();
                let left = Self::dfs(&root.left, answer);
                let right = Self::dfs(&root.right, answer);
                let sum = left.sum + right.sum + root.val;
                let size = left.size + right.size + 1;
                if (sum as f64 / size as f64).floor() as i32 == root.val {
                    *answer += 1;
                }
                return Container {
                    size: size,
                    sum: sum,
                }
            }
            None => Container {
                size: 0,
                sum: 0,
            }
        }
    }
} 
