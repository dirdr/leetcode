impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut nums = nums;
        let primes = Self::generate_prime_to_n(*nums.iter().max().unwrap());
        let minimize_low_bound = |num: i32, bound: i32| {
            for i in (0..primes.len()).rev() {
                if primes[i] < num && (num - primes[i]) > bound {
                    return num - primes[i];
                }
            }
            -1
        };
        let minimized = minimize_low_bound(nums[0], 0);
        if minimized != -1 {
            nums[0] = minimized;
        }
        for i in 1..nums.len() {
            let minimized = minimize_low_bound(nums[i], nums[i - 1]);
            if minimized != -1 {
                nums[i] = minimized;
            }
            if nums[i] <= nums[i - 1] {
                return false;
            }
        }
        true
    }

    fn generate_prime_to_n(n: i32) -> Vec<i32> {
        if n <= 2 {
            return vec![];
        }
        let n = n as usize;
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;
        let sqrt_n = (n as f64).sqrt() as usize;
        for i in 2..=sqrt_n {
            if is_prime[i] {
                let mut j = i * i;
                while j < n {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        is_prime
            .iter()
            .enumerate()
            .skip(2)
            .filter(|(_, &is_prime)| is_prime == true)
            .map(|(e, _)| e as i32)
            .collect::<Vec<i32>>()
    }
}
