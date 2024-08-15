impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut five: i32 = 0;
        let mut ten: i32 = 0;
        for bill in bills {
            if bill == 5 {
                five += 1;
            }
            else if bill == 10 {
                ten += 1;
                if five > 0 { five -= 1; }
                else {
                    return false;
                }
            }
            else if bill == 20 {
                if ten > 0 && five > 0 {
                    ten -= 1;
                    five -= 1;
                }
                else if five >= 3 {
                    five -= 3;
                }
                else {
                    return false;
                }
            }
        }
        return true;
    }
}
