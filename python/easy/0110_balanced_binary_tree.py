# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def dfs(root: Optional[TreeNode]) -> (bool, int):
            if root is None:
                return (True, 0)
            lBalanced, lHeight = dfs(root.left)
            rBalanced, rHeight = dfs(root.right)
            if not lBalanced or not rBalanced:
                return (False, -1)
            return (abs(rHeight - lHeight) <= 1, max(lHeight, rHeight) + 1)
        balanced, height = dfs(root)
        return balanced
