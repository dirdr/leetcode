impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;
        while (left <= right) {
            let middle: i32 = ((left + right) / 2);
            if nums[middle as usize] == target {return middle};
            if target > nums[middle as usize] {
                left = middle + 1;
            } else {
                right = middle - 1;
            }
        }
        let temp = left + right;
        if temp % 2 == 0 {return temp/2} else {return (temp + 1)/2}
    }
}
