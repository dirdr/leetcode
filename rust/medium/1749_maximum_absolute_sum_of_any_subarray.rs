impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let (mut min, mut max, mut sum, mut result) = (0, 0, 0, 0);
        for &el in &nums {
            sum += el;
            min = min.min(sum);
            max = max.max(sum);
            let first = sum - max;
            let second = sum - min;
            result = result.max(first.abs()).max(second.abs());
        }
        result
    }
}
