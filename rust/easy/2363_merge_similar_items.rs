use std::collections::HashMap;

impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        for i in 0..items1.len() {
            map.entry(items1[i][0])
                .and_modify(|e| {*e += items1[i][1]})
                .or_insert(items1[i][1]);
        }
        for i in 0..items2.len() {
            map.entry(items2[i][0])
                .and_modify(|e| {*e += items2[i][1]})
                .or_insert(items2[i][1]);
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        result =  map.iter().map(|t| {
            vec![*t.0, *t.1]
        }).collect();
        result.sort_by(|a, b| {
            a[0].cmp(&b[0])
        });
        return result;
    }
}
