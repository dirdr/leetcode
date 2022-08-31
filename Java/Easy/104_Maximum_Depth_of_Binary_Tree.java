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
    public int maxDepth(TreeNode root) {
        
        int answer = helper(root, 0);
        return answer;
        
    }
    
    public int helper(TreeNode root) {
        
        if (root == null) {
            return 0;
        }
        
        int left = helper(root.left);
        int right = helper(root.right, val);
        
        return Math.max(left, right)+1;
        
    }
}

/**
 * Runtime: 0 ms, faster than 100.00% of Java online submissions for Maximum Depth of Binary Tree.
 * Memory Usage: 43.8 MB, less than 5.00% of Java online submissions for Maximum Depth of Binary Tree.
*/
