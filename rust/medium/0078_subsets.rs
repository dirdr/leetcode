impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        Self::backtrack(&mut result, &mut Vec::new(), &nums[..], 0);
        result
    }

    pub fn backtrack(
        result: &mut Vec<Vec<i32>>,
        temp: &mut Vec<i32>,
        nums: &[i32],
        position: usize
    ) -> () {
        if (position >= temp.len()) {
            result.push(temp.clone());
            return;
        }
        temp.push(nums[position]);
        Solution::backtrack(result, temp, nums, position + 1);
        temp.pop();
        Solution::backtrack(result, temp, nums, position + 1);
    }
}
