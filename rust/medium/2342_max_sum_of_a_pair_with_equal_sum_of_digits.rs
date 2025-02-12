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
        let mut max_sum = 0;
        let mut map: HashMap<i32, i32> = HashMap::new();
        for &el in &nums {
            let digits_sum = sum_digits(el);
            if let Some(entry) = map.get_mut(&digits_sum) {
                let max_entry = el.max(*entry);
                max_sum = max_sum.max(*entry + el);
                *entry = el.max(*entry);
            } else {
                map.insert(digits_sum, el);
            }
        }
        if max_sum == 0 {
            return -1;
        }
        max_sum
    }
}
