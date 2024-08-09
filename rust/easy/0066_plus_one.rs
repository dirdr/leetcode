impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        let mut digits = digits;
        let mut flag = false;
        for (i, el) in digits.iter_mut().rev().enumerate() {
            let mut new = *el + 1;
            if new <= 9 {
                *el = new;
                break;
            } else {
                if i + 1 == n {
                    flag = true;
                }
                *el = 0;
            }
        }
        if flag {
            digits.insert(0, 1);
        }
        digits
    }
}
