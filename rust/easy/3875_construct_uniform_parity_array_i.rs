impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        fn check(nums: &[i32], predicate: fn(i32) -> bool) -> bool {
            for i in 0..nums.len() {
                if predicate(nums[i]) {
                    continue
                }
                let mut possible = false;
                for j in 0..nums.len() {
                    if i == j {
                        continue;
                    }
                    if predicate(nums[i] + nums[i] - nums[j]) {
                        possible = true;
                        break;
                    }
                }
                if !possible {
                    return false;
                }
            }
            true
        }
        let even = check(&nums1, |n| n % 2 == 0);
        let odd = check(&nums1, |n| n % 2 != 0);
        odd || even
    }
}
