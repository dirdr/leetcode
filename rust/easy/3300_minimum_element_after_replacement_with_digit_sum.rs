impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        fn sum(mut num: i32) -> i32 {
            let mut s = 0;
            while num > 0 {
                s += num % 10;
                num /= 10;
            }
            s
        }
        nums.into_iter().map(|d| sum(d)).min().unwrap()
    }
}
