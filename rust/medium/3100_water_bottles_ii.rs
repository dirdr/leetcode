impl Solution {
    pub fn max_bottles_drunk(mut bottles: i32, mut exchange: i32) -> i32 {
        let mut count = 0;
        while bottles >= exchange {
            count += exchange;
            bottles -= exchange;
            bottles += 1;
            exchange += 1;
        }
        count + bottles
    }
}
