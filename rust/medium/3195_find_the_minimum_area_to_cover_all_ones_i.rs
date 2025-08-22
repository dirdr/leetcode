impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut min_x, mut max_x) = (i32::MAX, i32::MIN);
        let (mut min_y, mut max_y) = (i32::MAX, i32::MIN);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 1 {
                    continue;
                }
                min_x = min_x.min(i as i32);
                min_y = min_y.min(j as i32);
                max_x = max_x.max(i as i32);
                max_y = max_y.max(j as i32);
            }
        }
        (max_y - min_y + 1) * (max_x - min_x + 1)
    }
}
