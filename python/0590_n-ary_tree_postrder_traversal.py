"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        l = []
        def dfs(root):
            for x in root.children:
                dfs(x)
            l.append(root.val)
        dfs(root)
        return l                
