impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut to_delete = 0;
        let strs = strs.iter().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        for c in 0..strs[0].len() {
            for r in 1..strs.len() {
                if strs[r][c] < strs[r - 1][c] {
                    to_delete += 1;
                    break;
                }
            }
        }
        to_delete
    }
}
