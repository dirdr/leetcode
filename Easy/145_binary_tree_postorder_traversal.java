/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    
    Wrapper wrapper = new Wrapper();
    public List<Integer> postorderTraversal(TreeNode root) {  
        traversal(root);
        return wrapper.list;
    }
    
    public void traversal(TreeNode root) {
        if (root == null) return;
        traversal(root.left);
        traversal(root.right);
        wrapper.list.add(root.val);
        
    }
}

class Wrapper {
    public ArrayList<Integer> list;
    public Wrapper() {
        list = new ArrayList<>();
    }
}
/**
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Binary Tree Postorder Traversal.
* Memory Usage: 39.2 MB, less than 6.31% of Java online submissions for Binary Tree Postorder Traversal.
*/
