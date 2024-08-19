use std::collections::HashSet;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n];
        dp[0] = 0;
        let primes = Solution::sieve_of_eratosthenes(n);
        for i in 1..n {
            let num = i as i32 + 1;
            if primes.contains(&num)  {
                dp[i] = num;
            } else {
                dp[i] = Solution::prime_factors(num).iter().map(|&e| dp[e as usize - 1]).sum::<i32>();
            }
        }
        dp[n - 1]
    }

    fn prime_factors(n: i32) -> Vec<i32> {
        let mut n = n;
        let mut factors = Vec::new();
        while n % 2 == 0 {
            factors.push(2);
            n /= 2;
        }
        let mut i = 3;
        while i * i <= n {
            while n % i == 0 {
                factors.push(i);
                n /= i;
            }
            i += 2;
        }
        if n > 2 {
            factors.push(n);
        }
        factors
    }

    fn sieve_of_eratosthenes(limit: usize) -> HashSet<i32> {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        let limit_sqrt = (limit as f64).sqrt() as usize;
        for i in 2..=limit_sqrt {
            if is_prime[i] {
                for multiple in (i * i..=limit).step_by(i) {
                    is_prime[multiple] = false;
                }
            }
        }

        is_prime.iter().enumerate()
            .filter_map(|(num, &is_prime)| if is_prime { Some(num as i32) } else { None })
            .collect()
    }
}
