impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        let break_digit = |mut num: usize| -> Vec<usize> {
            if num == 0 {
                return vec![0];
            }
            
            let mut digits = Vec::new();
            while num > 0 {
                digits.push(num % 10);
                num /= 10;
            }
            digits.reverse();
            digits
        };
        let s = s.into_bytes();
        let mut digits = s.iter()
            .map(|b| (b - b'a' + 1) as usize)
            .collect::<Vec<_>>();
        for _ in 0..k {
            let mut transformed = digits.iter()
                .map(|&e| break_digit(e))
                .flatten()
                .fold(0usize, |acc, x| acc + x);
            digits = break_digit(transformed);
        }
        digits.iter()
            .map(|&v| char::from_digit(v as u32, 10)
            .unwrap())
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}
