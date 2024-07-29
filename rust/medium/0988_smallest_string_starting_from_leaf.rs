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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        fn backtrack(node: &Option<Rc<RefCell<TreeNode>>>, curr: &mut String, min: &mut String) {
            if let Some(n) = node {
                let n = n.borrow();
                curr.push((n.val as u8 + b'a') as char);
                if n.left.is_none() && n.right.is_none() {
                    let reversed: String = curr.chars().rev().collect();
                    if min.is_empty() || reversed < *min {
                        *min = reversed;
                    }
                } else {
                    backtrack(&n.left, curr, min);
                    backtrack(&n.right, curr, min);
                }
                curr.pop();
            }
        }
        let mut min = String::new();
        backtrack(&root, &mut String::new(), &mut min);
        min
    }
}
