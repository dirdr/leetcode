impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut pairs = vec![];
        let m = potions.len();
        potions.sort();
        for (i, &s) in spells.iter().enumerate() {
            let i = potions.partition_point(|&p| (p as i64 * s as i64) < success);
            pairs.push((m - i) as i32);
        }
        pairs
    }
}
