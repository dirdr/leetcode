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
    

    public int distributeCoins(TreeNode root) {
        Wrapper wrapper = new Wrapper();
        dfs(root, wrapper);
        return wrapper.count;
    }
    
    public int dfs(TreeNode root, Wrapper wrapper) {
        
        if (root == null) return 0;
        
        int left = dfs(root.left, wrapper);
        int right = dfs(root.right, wrapper);
        
        wrapper.count += Math.abs(left) + Math.abs(right);
        
        return root.val + left + right -1;
        
    }
}

class Wrapper {
    
    public int count;
    
    public Wrapper() {
        this.count = 0;
    }
}

/*
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Distribute Coins in Binary Tree.
* Memory Usage: 41.4 MB, less than 87.65% of Java online submissions for Distribute Coins in Binary Tree.
*/
