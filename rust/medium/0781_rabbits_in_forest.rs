impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        for &el in &answers {
            map.entry(el).and_modify(|e| *e += 1).or_insert(1);
        }
        let mut answer = 0;
        for (k, v) in map.into_iter() {
            let group_size = k + 1;
            let group_needed = (v as f32 / group_size as f32).ceil() as i32;
            answer += group_needed * group_size;
        }
        answer
    }
}
