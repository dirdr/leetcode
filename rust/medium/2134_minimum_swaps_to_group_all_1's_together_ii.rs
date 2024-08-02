impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let window_size = nums.iter().filter(|&&e| e == 1).count();
        if window_size == 0 {
            return 0;
        }
        let mut copy = nums.clone();
        let mut nums = nums;
        nums.append(&mut copy);
        let mut count = 0;
        for i in 0..window_size {
            if nums[i] == 0 {
                count += 1;
            }
        }
        if window_size == nums.len() {
            return 0;
        }
        let mut min = count;
        for i in window_size..(nums.len()) {
            count += (nums[i - window_size] - nums[i]);
            min = min.min(count);
        }
        min
    }
}
