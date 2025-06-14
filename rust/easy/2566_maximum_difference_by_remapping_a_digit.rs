impl Solution {
    pub fn min_max_difference(num: i32) -> i32 {
        let decompose_digits = |mut a: i32| {
            let mut digits = vec![];
            while a > 0 {
                digits.push(a % 10);
                a /= 10;
            }
            digits
        };

        let mut digits = decompose_digits(num);
        digits.reverse();

        let (min_remap, mut max_remap) = (digits[0], -1);
        for &d in &digits {
            if d != 9 {
                max_remap = d;
                break;
            }
        }

        let mut mul = 1;
        let (mut a, mut b) = (0, 0);
        let mut diff = 0;
        for &digit in digits.iter().rev() {
            let a = if digit == max_remap { 9 } else { digit };
            let b = if digit == min_remap { 0 } else { digit };
            diff += (a - b) * mul;
            mul *= 10;
        }
        diff
    }
}
