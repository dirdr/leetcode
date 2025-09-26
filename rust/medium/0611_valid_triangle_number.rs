impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut count = 0;
        for k in (2..nums.len()).rev() {
            let mut i = 0;
            let mut j = k - 1;
            while i < j {
                if nums[i] + nums[j] > nums[k] {
                    count += (j - i) as i32;
                    j -= 1;
                } else {
                    i += 1;
                }
            }
        }
        count 
    }
}
