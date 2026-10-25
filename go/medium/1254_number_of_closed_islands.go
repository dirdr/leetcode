func dfs(grid [][]int, r int, c int, ban map[[2]int]struct{}) bool {
    if r < 0 || r >= len(grid) || c < 0 || c >= len(grid[0]) {
        return false
    }

    if grid[r][c] == 1 {
        return true
    }

    key := [2]int{r, c}

    if _, ok := ban[key]; ok {
        return true
    }

    ban[key] = struct{}{}

    down := dfs(grid, r+1, c, ban)
    up := dfs(grid, r-1, c, ban)
    right := dfs(grid, r, c+1, ban)
    left := dfs(grid, r, c-1, ban)

    return down && up && right && left
}

func closedIsland(grid [][]int) int {
    ban := make(map[[2]int]struct{})
    count := 0
    for i := 0; i < len(grid); i++ {
        for j := 0; j < len(grid[0]); j++ {
            if grid[i][j] == 0 {
                key := [2]int{i, j}
                if _, ok := ban[key]; !ok {
                    if dfs(grid, i, j, ban) {
                        count++
                    }
                }
            }
        }
    }

    return count
}
