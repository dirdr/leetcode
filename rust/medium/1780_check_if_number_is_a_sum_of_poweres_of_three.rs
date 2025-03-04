impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut result = vec![];
        let mut n = n;
        loop {
            let m = n % 3;
            n = n / 3;
            result.push(std::char::from_digit(m as u32, 3).unwrap());
            if n == 0 {
                break;
            }
        }       
        result.into_iter().all(|e| e != '2')
    }
}
