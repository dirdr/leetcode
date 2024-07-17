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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut answer = vec![];
        fn backtrack(root: &Option<Rc<RefCell<TreeNode>>>, current: &mut Vec<i32>, answer: &mut Vec<Vec<i32>>, sum: i32, target: i32) {
            if let Some(root) = root {
                let root = root.borrow();
                current.push(root.val);
                if root.left.is_none() && root.right.is_none() && sum + root.val == target {
                    answer.push(current.clone());
                } else {
                    backtrack(&root.left, current, answer, sum + root.val, target);
                    backtrack(&root.right, current, answer, sum + root.val, target);
                }
                current.pop();
            }
        }
        backtrack(&root, &mut vec![], &mut answer, 0, target_sum);
        answer
    }
}
