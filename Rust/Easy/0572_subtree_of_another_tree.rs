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
    // the two way i solved this :
    // just pre order (or post order, or in order) checking if current == sub_root as TreeNode derive Eq,
    // i just did pre order here because that can cut some case when the sub_tree is closer to the root
    // and avoid unnecessary calculations
    // or we can use a recursive function same_tree to perform the Eq check if that was not implemented.
    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, sub_root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if root == sub_root {
                return true;
            }
            match root {
                Some(node) => {
                    dfs(&node.borrow().left, sub_root) ||
                    dfs(&node.borrow().right, sub_root)
                },
                None => false
            }
        }
        dfs(&root, &sub_root)
    }
}
