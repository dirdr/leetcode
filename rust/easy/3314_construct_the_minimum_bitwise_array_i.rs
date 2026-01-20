impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![-1; n];
        for (i, &el) in nums.iter().enumerate() {
            for j in 0..el {
                if j | (j + 1) == el {
                    ans[i] = j;
                    break;
                }
            }
        }
        ans
    }
}
