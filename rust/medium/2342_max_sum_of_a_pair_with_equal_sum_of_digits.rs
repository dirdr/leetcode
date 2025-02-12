use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut sum_digits = |num: i32| {
            let mut sum = 0;
            let mut num = num;
            while num > 0 {
                let digit = num % 10;
                sum += digit;
                num /= 10;
            }
            sum
        };
        let mut max = 0;
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for &el in &nums {
            let digits_sum = sum_digits(el);
            if let Some(list) = map.get_mut(&digits_sum) {
                for &cand in list.iter() {
                    max = max.max(cand + el);
                }
                list.push(el);
            } else {
                map.insert(digits_sum, vec![el]);
            }
        }
        if max == 0 {
            return -1;
        }
        max
    }
}
