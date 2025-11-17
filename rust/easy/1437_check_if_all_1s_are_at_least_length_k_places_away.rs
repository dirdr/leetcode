impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let nums = nums.into_iter().skip_while(|e| *e != 1).collect::<Vec<_>>();
        let mut distance = -1;
        for &el in &nums {
            if el == 1 {
                if distance < k && distance != -1 {
                    return false;
                }
                distance = 0;
            } else {
                distance += 1;
            }
        }
        true
    }
}
