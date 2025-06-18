impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let jump = n / (n / 3);
        let mut answer = vec![];
        let mut start = 0;
        while start < n - jump + 1 {
            let array = nums[start..start + jump].to_vec();
            if array[array.len() - 1] - array[0] > k {
                return vec![];
            }
            answer.push(array);
            start = start + jump;
        }
        answer
    }
}
