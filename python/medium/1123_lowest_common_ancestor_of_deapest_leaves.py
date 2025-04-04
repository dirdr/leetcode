# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def lcaDeepestLeaves(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfs(root: Optional[TreeNode]):
            if root is None:
                return (0, None)

            leftH, leftN = dfs(root.left)
            rightH, rightN = dfs(root.right)
            
            if leftH > rightH:
                return (leftH + 1, leftN)

            if rightH > leftH:
                return (rightH + 1, rightN)

            return (leftH + 1, root)

        return dfs(root)[1]
