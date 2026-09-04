impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut pref = vec![0; n];
        let mut suff = vec![0; n];

        pref[0] = nums[0];
        suff[n - 1] = nums[n - 1];
        
        for i in 1..n {
            pref[i] = pref[i - 1].max(nums[i]);
        }

        for i in 1..n {
            let j = n - i - 1;
            suff[j] = suff[j + 1].min(nums[j]);
        }

        for (i, p) in pref.iter().enumerate() {
            if p - suff[i] <= k {
                return i as i32;
            }
        }

        -1
    }
}
