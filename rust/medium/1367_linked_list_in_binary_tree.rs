use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut nums = vec![];
        let mut current = head;
        while let Some(node) = current {
            nums.push(node.val);
            current = node.next;
        }

        fn dfs(nums: &[i32], root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if nums.is_empty() {
                return true;
            }
            match root {
                None => false,
                Some(node) => {
                    let node = node.borrow();
                    if node.val == nums[0] && is_path(nums, root) {
                        return true;
                    }
                    dfs(nums, &node.left) || dfs(nums, &node.right)
                }
            }
        }

        fn is_path(nums: &[i32], root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
            if nums.is_empty() {
                return true;
            }
            match root {
                None => false,
                Some(node) => {
                    let node = node.borrow();
                    if node.val != nums[0] {
                        return false;
                    }
                    if nums.len() == 1 {
                        return true;
                    }
                    is_path(&nums[1..], &node.left) || is_path(&nums[1..], &node.right)
                }
            }
        }
        dfs(&nums, &root)
    }
}
