impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        fn rev(mut num: i32) -> i32 {
            let mut digits = vec![];
            while num > 0 {
                digits.push(num % 10);
                num /= 10;
            }
            let mut out = 0;
            let mut mul = 1;
            for el in digits.iter().rev() {
                out += (el * mul);
                mul *= 10;
            }
            out
        }
        (n - rev(n)).abs()
    }
}
