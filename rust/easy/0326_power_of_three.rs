impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let log = (n as f64).log(3f64);
        let nearest = log.round();
        (log - nearest).abs() <= 1e-12 * nearest.abs().max(1.0)
    }
}
