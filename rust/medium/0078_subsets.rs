impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32], answer: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, start: usize) {
            answer.push(temp.clone());
            for i in start..nums.len() {
                temp.push(nums[i]);
                helper(nums, answer, temp, i + 1);
                temp.pop();
            }
        }
        let mut answer = vec![];
        helper(&nums[..], &mut answer, &mut vec![], 0);
        answer
    }
}
