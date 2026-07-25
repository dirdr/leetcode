impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let mut max = 0;
        let mut m = n;
        let mut i = 0;
        let mut max_i = 0;
        while m > 0 {
            if m % 10 > max {
                max_i = i;
                max = m % 10;
            }
            m /= 10;
            i += 1;
        }
        let mut out = 0;
        let mut j = 0;
        while n > 0 {
            if j != max_i {
                out = out.max(max * (n % 10));
            }
            n /= 10;
            j += 1;
        }
        out
    }
}
