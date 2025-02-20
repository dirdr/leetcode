use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let set = HashSet::from_iter(nums.iter().cloned());
        fn backtrack(current: &mut Vec<char>, set: &HashSet<String>, n: usize) -> Option<String> {
            if current.len() == n {
                let owned =  current.iter().cloned().collect::<String>();
                if !set.contains(&owned) {
                    return Some(owned);
                } else {
                    return None;
                }
            }
            let mut unique = None;
            for &ch in &['0', '1'] {
                current.push(ch);
                if let Some(u) = backtrack(current, set, n) {
                    unique = Some(u);
                    break;
                }
                current.pop();
            }
            unique
        }
        backtrack(&mut vec![], &set, nums[0].len()).unwrap()
    }
}
