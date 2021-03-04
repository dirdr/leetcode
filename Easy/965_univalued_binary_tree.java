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
    public boolean isUnivalTree(TreeNode root) {
      
        // si jamais l'enfant a gauche est egal a la valeur du noeud actuel alors la condition est vrai et on descend d'un niveau dans la recursion 
        boolean left = (root.left == null || root.left.val == root.val && isUnivalTree(root.left));
        // si jamais l'enfant a droite est egal a la valeur du noeud actuel alors la condition est vrai et on descend d'un niveau dans la recursion 
        boolean right = (root.right == null || root.right.val == root.val && isUnivalTree(root.right));
      
      
        //pour que la condition soit vrai cad que l'arbre ai bien la même valeur partout on veut left && right sinon return false
     
        return left && right;
    }
}

/**
 * Runtime: 0 ms, faster than 100.00% of Java online submissions for Univalued Binary Tree.
 * Memory Usage: 36.8 MB, less than 30.91% of Java online submissions for Univalued Binary Tree.
 */
