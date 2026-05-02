impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        fn is_good(mut num: i32) -> bool {
            let mut flag = false;
            while num > 0 {
                let digit = num % 10;
                if digit == 3 || digit == 4 || digit == 7 {
                    return false;
                }
                if digit == 2 || digit == 5 || digit == 6 || digit == 9 {
                    flag = true
                }
                num /= 10;
            }
            flag
        }
        (1..=n).map(|num| is_good(num)).filter(|&e| e == true).count() as i32
    }
}
