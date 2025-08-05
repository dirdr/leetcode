impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut used = vec![false; baskets.len()];
        let mut unplaced = fruits.len() as i32;
        for &fruit in &fruits {
            for (i, &basket) in baskets.iter().enumerate() {
                if !used[i] && basket >= fruit {
                    unplaced -= 1;
                    used[i] = true;
                    break;
                }
            }
        }
        unplaced
    }
}
