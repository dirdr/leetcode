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
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) -> () {
        let mut order: Vec<i32> = vec![];
        Self::dfs(&root, &mut order);
        order.reverse(); // reverse because pop take the last
        std::mem::replace(root, None);
        Self::reorder(root, &mut order);
    }

    pub fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) -> () {
        match root {
            Some(r) => {
                stack.push(r.borrow().val);
                Self::dfs(&r.borrow().left, stack);
                Self::dfs(&r.borrow().right, stack);
            },
            None => return,
        }
    }

    pub fn reorder(root: &mut Option<Rc<RefCell<TreeNode>>>, order: &mut Vec<i32>) -> () {
        if order.is_empty() {return}
        match root {
            Some(r) => {
                r.borrow_mut().left = None;
                r.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(order.pop().unwrap()))));
                Self::reorder(&mut r.borrow_mut().right, order);
            },
            None => {
                *root = Some(Rc::new(RefCell::new(TreeNode::new(order.pop().unwrap()))));
                Self::reorder(root, order);
            },
        }
    }
}
