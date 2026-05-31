impl Solution {
    pub fn asteroids_destroyed(mut mass: i32, mut asteroids: Vec<i32>) -> bool {
        let mut sum: i64 = mass as i64;
        asteroids.sort_unstable();
        for &astr in &asteroids {
            if sum < astr as i64 {
                return false;
            } else {
                sum += astr as i64;
            }
        }
        true
    }
}
