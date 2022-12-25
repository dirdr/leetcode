use std::collections::HashMap;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for (pos, el) in arr.iter().cloned().enumerate() {
            map.insert(pos, el);
        }
        arr.sort();
        let mut current_rank: i32 = 1;
        let mut rank = HashMap::new();
        let len = arr.len();
        for i in 0..len {
            if !rank.contains_key(&arr[i]) {
                rank.insert(arr[i], current_rank);
                current_rank += 1;
            }
        }
        for i in 0..len {
            let el = (*rank.get(&map.get(&i).unwrap()).unwrap()) as i32;
            arr[i] = el;
        }
        arr
    }
}
