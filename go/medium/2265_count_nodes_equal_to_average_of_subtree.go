/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func averageOfSubtree(root *TreeNode) int {
    _, _, tc := dfs(root)
    return tc
}

func dfs(root *TreeNode) (int, int, int) {
    if root == nil {
        return 0, 0, 0
    }
    ls, ld, lc := dfs(root.Left)
    rs, rd, rc := dfs(root.Right)
    average := (ls  + rs + root.Val) / (ld + rd + 1)
    bonus := 0
    if root.Val == average {
        bonus = 1
    }
    return ls + rs + root.Val, ld + rd + 1, lc + rc + bonus
}
