impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        fn is_feasible(ranks: &[i64], cars: i64, time: i64) -> bool {
            let mut capacity = 0;
            for &r in ranks {
                capacity += (time as f64 / r as f64).sqrt().floor() as i64;
                if capacity >= cars {
                    return true;
                }
            }
            false
        }
        let cars = cars as i64;
        let ranks = ranks.into_iter().map(|r| r as i64).collect::<Vec<_>>();
        let (mut left, mut right) = (1, ranks.iter().min().unwrap() * (cars * cars));
        let mut result = right;
        while left <= right {
            let middle = left + (right - left) / 2;
            if is_feasible(&ranks, cars, middle) {
                result = middle;
                right = middle - 1
            } else {
                left = middle + 1
            }
        }
        result
    }
}
