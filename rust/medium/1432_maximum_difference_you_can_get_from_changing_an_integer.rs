impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let digits = Self::digits(num);
        
        let max_val = digits.iter()
            .find(|&&d| d != 9)
            .map_or(num, |&target| Self::replace_all(num, target, 9));
            
        let min_val = if digits[0] != 1 {
            Self::replace_all(num, digits[0], 1)
        } else {
            digits.iter().skip(1)
                .find(|&&d| d != 0 && d != 1)
                .map_or(num, |&target| Self::replace_all(num, target, 0))
        };
        
        max_val - min_val
    }
    
    fn digits(mut n: i32) -> Vec<i32> {
        std::iter::from_fn(move || {
            if n == 0 { None } else {
                let digit = n % 10;
                n /= 10;
                Some(digit)
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
    }
    
    fn replace_all(mut num: i32, from: i32, to: i32) -> i32 {
        let mut result = 0;
        let mut power = 1;
        
        while num > 0 {
            let digit = num % 10;
            result += (if digit == from { to } else { digit }) * power;
            power *= 10;
            num /= 10;
        }
        
        result
    }
}
