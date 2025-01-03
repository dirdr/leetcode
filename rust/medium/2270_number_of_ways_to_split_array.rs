impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut prefix = Vec::with_capacity(n);
        let mut suffix = Vec::with_capacity(n);
        let (mut psum, mut ssum) = (0, 0);
        for i in 0..n {
            psum += nums[i] as i64;
            ssum += nums[n - i - 1] as i64;
            prefix.push(psum);
            suffix.push(ssum);
        }
        suffix.reverse();
        let mut count = 0;
        for i in 0..n - 1 {
            if prefix[i] >= suffix[i + 1] {
                count += 1;
            }
        }
        count
    }
}
