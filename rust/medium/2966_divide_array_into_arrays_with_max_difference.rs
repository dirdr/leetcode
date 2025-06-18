impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut answer = vec![];
        
        for i in (0..=nums.len() - 3).step_by(3) {
            let array = nums[i..i + 3].to_vec();
            if array[array.len() - 1] - array[0] > k {
                return vec![];
            }
            answer.push(array);
        }
        answer
    }
}
