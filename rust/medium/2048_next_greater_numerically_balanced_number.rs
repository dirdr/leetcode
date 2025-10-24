impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        fn is_balanced(mut num: i32) -> bool {
            let mut count = vec![0; 10];
            while num > 0 {
                let digit = num % 10;
                if digit == 0 {
                    return false;
                }
                count[digit as usize] += 1;
                num /= 10;
            }
            for i in 1..=9 {
                if count[i] != i && count[i] > 0 {
                    return false;
                }
            }
            true
        }
        for num in (n + 1)..=1224444 {
            if is_balanced(num) {
                return num;
            }
        }
        unreachable!()
    }
}
