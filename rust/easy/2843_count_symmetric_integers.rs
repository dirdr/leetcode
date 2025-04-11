impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        (low..=high)
            .filter(|&num| {
                let digits: Vec<u32> = num.to_string()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();
                
                if digits.len() % 2 != 0 {
                    return false;
                }
                
                let mid = digits.len() / 2;
                let left_sum: u32 = digits.iter().take(mid).sum();
                let right_sum: u32 = digits.iter().skip(mid).sum();
                
                left_sum == right_sum
            })
            .count() as i32
    }
}
