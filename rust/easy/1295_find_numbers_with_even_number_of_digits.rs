impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        fn get_digit_count(num: i32) -> usize {
            let mut num = num;
            let mut count = 0;
            while num > 0 {
                num /= 10;
                count += 1
            }
            count
        }
        nums.into_iter()
            .filter(|&num| get_digit_count(num) % 2 == 0)
            .count() as i32
    }
}
