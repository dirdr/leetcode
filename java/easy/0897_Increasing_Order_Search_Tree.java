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
    
    ArrayList<Integer> list = new ArrayList<>();
    
    public TreeNode increasingBST(TreeNode root) {
        helper(root);
        TreeNode newRoot = new TreeNode(list.get(0));
        TreeNode current = newRoot;
        for (int i = 1; i < list.size(); i++) {
            current.right = new TreeNode(list.get(i));
            current = current.right;
        }
        return newRoot;
    }
    public void helper(TreeNode root) {
        if (root == null) return;
        helper(root.left);
        list.add(root.val);
        helper(root.right);
    }
}

/*
* Runtime: 1 ms, faster than 38.39% of Java online submissions for Increasing Order Search Tree.
* Memory Usage: 41.5 MB, less than 55.86% of Java online submissions for Increasing Order Search Tree.
*/
