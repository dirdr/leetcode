impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        fn sum(mat: &[Vec<i32>], len: usize, threshold: i32) -> bool {
            if len == 0 { return true; }
            for i in 0..=mat.len().saturating_sub(len) {
                for j in 0..=mat[0].len().saturating_sub(len) {
                    let mut sum = 0;
                    for di in 0..len {
                        for dj in 0..len {
                            sum += mat[i + di][j + dj];
                        }
                    }
                    if sum <= threshold {
                        return true;
                    }
                }
            }
            false
        }
        
        let (mut l, mut r) = (0, mat.len().min(mat[0].len()));
        while l < r {
            let mid = l + (r - l) / 2;
            if sum(&mat, mid + 1, threshold) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        l as i32
    }
}
