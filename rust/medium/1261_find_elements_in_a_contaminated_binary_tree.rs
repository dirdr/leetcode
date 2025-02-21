use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;

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
struct FindElements {
    set: HashSet<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, set: &mut HashSet<i32>, val: i32) {
            let root = match root {
                Some(root) => root,
                None => return,
            };

            let node = root.borrow();
            set.insert(val);

            if node.left.is_some() {
                dfs(&node.left, set, val * 2 + 1);
            }
            if node.right.is_some() {
                dfs(&node.right, set, val * 2 + 2);
            }
        }
        let mut set = HashSet::new();
        dfs(&root, &mut set, 0);
        Self {
            set,
        }
    }
    
    fn find(&self, target: i32) -> bool {
        self.set.contains(&target)
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */
