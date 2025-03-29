# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def diameterOfBinaryTree(self, root: Optional[TreeNode]) -> int:
        def dfs(root) -> (int, int):
            if root is None:
                return (0, 0)
            ld, lh = dfs(root.left)
            rd, rh = dfs(root.right)
            return (max(ld, rd, lh + rh), max(lh, rh) + 1)
        md, mh = dfs(root)
        return md
