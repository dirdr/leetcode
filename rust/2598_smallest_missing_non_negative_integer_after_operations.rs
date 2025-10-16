impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut mp = vec![0; value as usize];
        nums.iter().for_each(|&x| mp[x.rem_euclid(value) as usize] += 1);
        let mut mex = 0i32;
        while mp[(mex.rem_euclid(value)) as usize] > 0 {
            mp[(mex.rem_euclid(value)) as usize] -= 1;
            mex += 1;
        }
        mex
    }
}
