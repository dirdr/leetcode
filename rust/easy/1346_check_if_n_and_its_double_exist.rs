use std::collections::HashMap;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = HashMap::new();
        arr.iter().enumerate().for_each(|(i, el)| {
            map.insert(el * 2, i);
        });
        for (i, el) in arr.iter().enumerate() {
            match map.get(el) {
                Some(val) => if i != *val {return true},
                None => (),
            }
        }
        false
    }
}
