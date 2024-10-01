impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut freq = vec![0; k as usize];
        for el in &arr {
            let rem = ((el % k) + k) % k;
            freq[rem as usize] += 1;
        }
        let k = k as usize;
        for i in 1..k {
            if freq[i] != freq[k - i] {
                return false;
            }
        }
        if k % 2 == 0 {
            if freq[k as usize / 2] % 2 != 0 {
                return false;
            }
            if freq[0] % 2 != 0 {
                return false;
            }
        }
        true
    }
}
