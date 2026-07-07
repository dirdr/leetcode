impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut new = 0;
        let mut curr_mul = 1;
        let mut sum = 0;

        while n > 0 {
            let digit = n as i64 % 10;
            n /= 10;
            if digit > 0 {
                 sum += digit;
                new += (digit * curr_mul);
                curr_mul *= 10;
            }
        }
        
        new * sum
    }
}
