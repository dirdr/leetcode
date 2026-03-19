impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut pref = vec![vec![vec![0; 2]; n]; m];
        for i in 0..m {
            for j in 0..n {
                let (mut x_pref, mut y_pref) = (0, 0);
                let xd = if grid[i][j] == 'X' { 1 } else { 0 };
                let yd = if grid[i][j] == 'Y' { 1 } else { 0 };
                x_pref += xd;
                y_pref += yd;
                if i > 0 {
                    x_pref += pref[i - 1][j][0];
                    y_pref += pref[i - 1][j][1];
                }
                if j > 0 {
                    x_pref += pref[i][j - 1][0];
                    y_pref += pref[i][j - 1][1];
                }
                if i > 0 && j > 0 {
                    x_pref -= pref[i - 1][j - 1][0];
                    y_pref -= pref[i - 1][j - 1][1];
                }
                pref[i][j][0] = x_pref;
                pref[i][j][1] = y_pref;
            }
        }
        pref.into_iter().flatten().filter(|c| c[0] >= 1 && c[0] == c[1]).count() as i32
    }
}
