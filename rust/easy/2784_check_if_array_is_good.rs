impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let n = nums.len() - 1;
        let mut occ = vec![1; n];
        occ[n - 1] = 2;
        for &el in &nums {
            if el as usize - 1 >= n {
                return false;
            }
            occ[el as usize - 1] -= 1;
        }
        for &num in &occ {
            if num != 0 {
                return false;
            }
        }
        true
    }
}
