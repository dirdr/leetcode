use std::collections::HashMap;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        fn helper(grid: &Vec<Vec<i32>>, memo: &mut HashMap<(usize, usize), i32>, curr: (usize, usize)) -> i32 {
            let (r, c) = curr;
            let (width, height) = (grid.len(), grid[0].len());
            if let Some(min) = memo.get(&curr) {
                return *min;
            }
            let val = grid[r][c];
            if r + 1 == width && c + 1 == height {
                return val;
            }
            let down = if r + 1 < width { helper(grid, memo, (r + 1, c)) } else { i32::MAX };
            let right = if c + 1 < height { helper(grid, memo, (r, c + 1)) } else { i32::MAX };
            let min = down.min(right) + val;
            memo.insert(curr, min);
            min
        }
        helper(&grid, &mut HashMap::new(), (0, 0))
    }
}
