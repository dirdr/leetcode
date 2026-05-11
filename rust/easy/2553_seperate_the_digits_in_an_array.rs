impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().flat_map(|&e| {
            let mut e = e;
            let mut digits = vec![];
            while e > 0 {
                digits.push(e % 10);
                e /= 10;
            }
            digits.into_iter().rev()
        }).collect::<Vec<_>>()
    }
}
