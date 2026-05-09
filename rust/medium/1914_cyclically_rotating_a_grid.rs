impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut arrays = vec![];
        let (mut top, mut bottom, mut left, mut right) = (0, m - 1, 0, n - 1);
        while top <= bottom && left <= right {
            let mut current = vec![];
            // left column: top to bottom - 1
            for r in top..bottom {
                current.push(grid[r][left]);
            }
            // bottom row: left to right
            for c in left..=right {
                current.push(grid[bottom][c]);
            }
            // right column: bottom - 1 down to top
            for r in (top..=bottom - 1).rev() {
                current.push(grid[r][right]);
            }
            // top row: right - 1 down to left + 1
            for c in (left + 1..right).rev() {
                current.push(grid[top][c]);
            }
            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
            arrays.push(current);
        }
        let mut rotated = vec![];
        for array in arrays {
            let mut new = vec![0; array.len()];
            for i in 0..array.len() {
                new[(i + k as usize) % array.len()] = array[i];
            }
            rotated.push(new);
        }
        let (mut top, mut bottom, mut left, mut right) = (0, m - 1, 0, n - 1);
        let mut curr = 0;
        while top <= bottom && left <= right {
            let mut idx = 0;
            // left column: top to bottom - 1
            for r in top..bottom {
                grid[r][left] = rotated[curr][idx];
                idx += 1;
            }
            // bottom row: left to right
            for c in left..=right {
                grid[bottom][c] = rotated[curr][idx];
                idx += 1;;
            }
            // right column: bottom - 1 down to top
            for r in (top..=bottom - 1).rev() {
                grid[r][right] = rotated[curr][idx];
                idx += 1;
            }
            // top row: right - 1 down to left + 1
            for c in (left + 1..right).rev() {
                grid[top][c] = rotated[curr][idx];
                idx += 1;
            }
            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
            curr += 1;
        }
        grid
    }
}
