use std::collections::HashMap;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        fn rev(mut num: i32) -> i32 {
            let mut digits = vec![];
            while num > 0 {
                let d = num % 10;
                digits.push(d);
                num /= 10;
            }
            let mut out = 0;
            let mut mul = 1;
            for el in digits.iter().rev().skip_while(|&&d| d == 0) {
                out += el * mul;
                mul *= 10;
            }
            out
        }
        let mut map: HashMap<i32, usize> = HashMap::new();
        let mut min = i32::MAX;
        for (i, &el) in nums.iter().enumerate() {
            if el == 0 {
                continue;
            }
            let reverse = rev(el);
            if let Some(&j) = map.get(&el) {
                min = min.min((i as i32 - j as i32).abs());
            }
            map.insert(reverse, i);
        }
        if min == i32::MAX {
            return -1;
        }
        min
    }
}
