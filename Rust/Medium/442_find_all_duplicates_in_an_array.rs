/* O(N) Time and O(N) Space complexity
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        nums.iter().for_each(|int| {
            map.entry(int).and_modify(|e| *e += 1).or_insert(1);
        });
        map.iter() // use for double dereference because iter give &T and filter give &T over &T -> &&T
            .filter(|(_, value)| **value == 2)
            .map(|(key, _)| **key)
            .collect::<Vec<i32>>()
    }
}
*/
impl Solution { // O(N) time O(1) complexity (return list not in space complexity)
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        // all the elements are in range [1, n] (array marking)??
        // each integer appear once or twice -> find one that appear twice
        let mut answer = Vec::new(); 
        for i in (0..nums.len()) {
            let mut index = (nums[i].abs() - 1) as usize;
            if nums[index] < 0 {
                answer.push(index as i32 + 1);
            } else {
                nums[index] *= -1;
            }
        }
        answer
    }
}
