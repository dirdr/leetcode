use std::collections::HashSet;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        let n = digits.len();
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if i != j && j != k && i != k {
                        let mut num = 0;
                        num += digits[k];
                        num += digits[j] * 10;
                        num += digits[i] * 100;
                        if num % 2 == 0 && num >= 100 {
                            set.insert(num);
                        }
                    }
                }
            }
        }
        set.len() as i32
    }
}
