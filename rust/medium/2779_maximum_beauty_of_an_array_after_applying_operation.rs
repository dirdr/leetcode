impl Solution {
    pub fn maximum_beauty(nums: Vec<i32>, k: i32) -> i32 {
        let mut events = vec![];
        for el in &nums {
            events.push((el - k, 1));
            events.push((el + k + 1, -1));
        }
        events.sort();
        let mut beauty = 0;
        let mut max = 0;
        for (value, effect) in &events {
            beauty += effect;
            max = max.max(beauty);
        }
        max
    }
}
