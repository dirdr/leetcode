impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let fact = |mut fact: usize| -> i64 {
            const MODULO: i64 = 1_000_000_007;
            let mut total = 1i64;
            while fact > 1 {
                total = (total * fact as i64) % MODULO;
                fact -= 1;
            }
            total
        };
        // since the first complexity will be unlocked, if any complexity is less than c[0], it can never been unlocked
        // If everything is more complex than the first one, we can unlock everything with the first computer unlocked
        if complexity.iter().skip(1).any(|&d| d <= complexity[0]) {
            0
        } else {
            fact(complexity.len() - 1) as i32
        }
    }
}
