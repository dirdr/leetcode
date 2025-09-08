impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let have_zero = |mut num| {
            while num > 0 {
                if num % 10 == 0 {
                    return true;
                }
                num /= 10;
            }
            false
        };
        let mut rem = 1;
        let mut n = n;
        while have_zero(n - rem) || have_zero(rem) {
            rem += 1;
        }
        vec![n - rem, rem]
    }
}
