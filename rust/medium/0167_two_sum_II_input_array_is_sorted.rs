impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0 as usize, numbers.len() - 1);
        while (left < right) {
            let sum = numbers[left] + numbers[right];
            if sum == target {
                return vec![left as i32 + 1, right as i32 + 1];
            }
            if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        vec![]
    }
}
