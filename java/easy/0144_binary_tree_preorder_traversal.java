class Solution {
    
    Wrapper wrapper = new Wrapper();
    public List<Integer> preorderTraversal(TreeNode root) {  
        traversal(root);
        return wrapper.list;
    }
    
    public void traversal(TreeNode root) {
        if (root == null) return;
        wrapper.list.add(root.val);
        traversal(root.left);
        traversal(root.right);
        
    }
}

class Wrapper {
    public ArrayList<Integer> list;
    public Wrapper() {
        list = new ArrayList<>();
    }
}

/**
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Binary Tree Preorder Traversal.
* Memory Usage: 39 MB, less than 11.68% of Java online submissions for Binary Tree Preorder Traversal.
*/
