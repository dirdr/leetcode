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
    
    int sum = 0;
    
    public int sumEvenGrandparent(TreeNode root) {
        helper(root);
        return sum;
    }
    
    public int helper(TreeNode root) {
        
        if (root == null) return 0;
        
        if (root.val % 2 == 0) {
            
            if (root.left != null && root.left.left != null) sum += root.left.left.val;
            if (root.left != null && root.left.right != null) sum += root.left.right.val;
            if (root.right != null && root.right.left != null) sum += root.right.left.val;
            if (root.right != null && root.right.right != null) sum += root.right.right.val;
            
        }
        
        int left = helper(root.left);
        int right = helper(root.right);
        
        return sum;
    }
}

/*
* Runtime: 2 ms, faster than 79.20% of Java online submissions for Sum of Nodes with Even-Valued Grandparent.
* Memory Usage: 52.6 MB, less than 32.59% of Java online submissions for Sum of Nodes with Even-Valued Grandparent.
*/
