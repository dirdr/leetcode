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
    
    public TreeNode bstToGst(TreeNode root) { // we choose inorder traversal because we need right -> root -> left 
        if (root == null) return null;
        
        TreeNode right = bstToGst(root.right);
        int temp = root.val;
        root.val += sum;
        sum += temp;
        TreeNode left = bstToGst(root.left);
        
        return root;
    }
}

/*
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Binary Search Tree to Greater Sum Tree.
* Memory Usage: 42.5 MB, less than 6.19% of Java online submissions for Binary Search Tree to Greater Sum Tree.
*/
