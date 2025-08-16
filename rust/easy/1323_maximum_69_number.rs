impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut digits = vec![];
        let mut num = num;
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        if let Some(digit) = digits.iter_mut().rev().find(|&&mut digit| digit == 6) {
            *digit = 9;
        }
        let mut answer = 0;
        let mut curr = 1;
        for i in 0..digits.len() {
            answer += digits[i] * curr;
            curr *= 10;
        }
        answer
    }
}
