use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut answer = vec![];   
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, current: &mut Vec<String>, answer: &mut Vec<String>) {
            if let Some(node) = root {
                let node = node.borrow();
                current.push(node.val.to_string());
                if node.left.is_none() && node.right.is_none() {
                    answer.push(current.join("->"));
                } else {
                    dfs(&node.left, current, answer);
                    dfs(&node.right, current, answer);
                } 
                current.pop();
            }
        }
        dfs(&root, &mut vec![], &mut answer);
        answer
    }
}
