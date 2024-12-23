use std::collections::VecDeque;
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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(root) = root else {
            return -1;
        };
        let mut queue = VecDeque::new();
        queue.push_front(root);
        let mut total = 0;
        while !queue.is_empty() {
            let mut next = VecDeque::new();
            let mut container = vec![];
            while let Some(node) = queue.pop_front() {
                let node = node.borrow();
                container.push(node.val);
                if let Some(ln) = node.left.clone() {
                    next.push_back(ln);
                }
                if let Some(rn) = node.right.clone() {
                    next.push_back(rn);
                }
            }
            total += Self::operations_to_sort_arr(&mut container);
            std::mem::swap(&mut queue, &mut next);
        }
        total
    }

    fn operations_to_sort_arr(nums: &mut Vec<i32>) -> i32 {
        let mut temp = nums.clone();
        temp.sort_unstable();
        let mut count = 0;
        for i in 0..nums.len() {
            if nums[i] != temp[i] {
                for k in i + 1..nums.len() {
                    if nums[k] == temp[i] {
                        count += 1;
                        let temp = nums[i];
                        nums[i] = nums[k];
                        nums[k] = temp;
                    }
                }
            }
        }
        count
    }
}
