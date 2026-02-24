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
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
            match root {
                None => vec![],
                Some(node) => {
                    let node = node.borrow();
                    let left = dfs(&node.left);
                    let right = dfs(&node.right);
                    let curr = if node.val == 1 { '1' } else { '0' };
                    if left.is_empty() && right.is_empty() {
                        vec![curr.to_string()]
                    } else {
                        let mut result = Vec::with_capacity(left.len() + right.len());
                        result.extend_from_slice(&left);
                        result.extend_from_slice(&right);
                        for s in result.iter_mut() {
                            s.push(curr);
                        }
                        result
                    }
                }
            }
        }
        fn bs_to_bn(binary: &str) -> i32 {
            let mut num = 0;
            for (i, ch) in binary.chars().enumerate() {
                if ch == '1' {
                    num |= 1 << i;
                }
            }
            num
        }
        let result = dfs(&root);
        result.into_iter().map(|s| bs_to_bn(&s)).sum::<i32>()
    }
}
