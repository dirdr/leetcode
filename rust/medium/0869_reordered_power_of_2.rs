use std::collections::HashSet;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        fn extract_digits(mut number: i32) -> [u8; 10] {
            let mut digits = [0; 10];
            while number > 0 {
                digits[number as usize % 10] += 1;
                number /= 10;
            }
            digits
        }
        let mut set = HashSet::new();
        let mut curr = 1;
        for i in 0..30 {
            set.insert(extract_digits(curr));
            curr = curr << 1;
        }
        set.contains(&extract_digits(n))
    }
}
