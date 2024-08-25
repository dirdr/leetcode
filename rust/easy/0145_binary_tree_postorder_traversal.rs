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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, answer: &mut Vec<i32>) {
            match root {
                None => return,
                Some(root) => {
                    let root = root.borrow();
                    dfs(&root.left, answer);
                    dfs(&root.right, answer);
                    answer.push(root.val);
                }
            }
        }
        let mut answer = vec![];
        dfs(&root, &mut answer);
        answer
    }
}
