use std::collections::HashSet;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn helper(nums: &[i32], answer: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>, start: usize) {
            answer.insert(temp.clone());
            for i in start..nums.len() {
                temp.push(nums[i]);
                helper(nums, answer, temp, i + 1);
                temp.pop();
            }
        }
        let mut answer = HashSet::new();
        let mut sorted = nums.clone();
        // sort the input let the hashset detect duplicate subset
        // that may be in different order without sorting (ex: [1, 4], [4, 1])
        sorted.sort();
        helper(&sorted, &mut answer, &mut vec![], 0);
        answer.into_iter().collect::<Vec<Vec<_>>>()
    }
}
