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
    
    public int deepestLeavesSum(TreeNode root) {
        
        int height = height(root); 
        helper(root, height);
        
        return sum;
    }
    
    public int helper(TreeNode root, int height) {
        
        if (root == null) return 0;
        if (height == 1) sum += root.val;
        
        int left = helper(root.left, height-1);
        int right = helper(root.right, height-1);
        
        return ++height;
    }
    
    public int height(TreeNode root) {
        
        if (root == null) return 0;
        
        int left = height(root.left);
        int right = height(root.right);
        
        return Math.max(left, right)+1;
    }
}

/*
* Runtime: 3 ms, faster than 76.65% of Java online submissions for Deepest Leaves Sum.
* Memory Usage: 56.1 MB, less than 48.88% of Java online submissions for Deepest Leaves Sum.
*/
