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
use std::collections::VecDeque;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = match root {
            None => return true,
            Some(r) => r,
        };
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();
        queue.push_back((root, 0));
        let mut current_level = 0;
        while (!queue.is_empty()) {
            let mut next: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();
            let mut size = 2_i32.pow(current_level);
            let mut container = vec![-101; size as usize];
            while let Some((node, index)) = queue.pop_front() {
                let node = node.borrow();
                if let Some(ln) = node.left.clone() {
                    next.push_back((ln, 2 * index));
                }
                if let Some(rn) = node.right.clone() {
                    next.push_back((rn, 2 * index + 1));
                }
                container[index] = node.val;
            }
            current_level += 1;
            if !Self::symmetric_slice(&container) {return false}
            std::mem::swap(&mut queue, &mut next);
        }
        true
    }

    pub fn symmetric_slice(vec: &Vec<i32>) -> bool {
        for i in 0..vec.len()/2 {
            if (vec[i] != vec[vec.len() - i - 1]) {return false}
        }
        true
    }
}
