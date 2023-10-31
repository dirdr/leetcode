use std::cmp::Ordering;

impl Solution {
    pub fn can_eat_all_in_time(k: i32, h: i32, piles: &[i32]) -> bool {
        let required_hours: f64 = piles.iter().map(|&e| f64::ceil(e as f64 / k as f64)).sum();
        required_hours <= h as f64
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut max_speed = *piles.iter().max().unwrap();
        let mut min_speed = 1;
        while min_speed <= max_speed {
            let middle = min_speed + (max_speed - min_speed) / 2;
            match Self::can_eat_all_in_time(middle, h, piles) {
                true => max_speed = middle - 1,
                false => min_speed = middle + 1,
            }
        }
        min_speed
    }
}
