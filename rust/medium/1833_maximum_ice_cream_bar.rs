impl Solution {
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        costs.sort();
        let mut answer = 0;
        for el in costs {
            if coins >= el {
                coins -= el;
                answer += 1;
            } else {break}
        }
        answer
    }
}
