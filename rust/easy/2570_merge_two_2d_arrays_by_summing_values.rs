use std::collections::BTreeMap;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut map: BTreeMap<i32, i32> = BTreeMap::new();
        let mut update = |pair_list: &Vec<Vec<i32>>| {
            for pair in pair_list {
                map.entry(pair[0])
                    .and_modify(|e| *e += pair[1])
                    .or_insert(pair[1]);
            }
        };
        update(&nums1);
        update(&nums2);
        map.into_iter()
            .map(|(k, v)| vec![k, v])
            .collect::<Vec<Vec<_>>>()
    }
}
