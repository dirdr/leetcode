impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        fn can_partition(num: i32, target: i32) -> bool {
            if num < target || target < 0 {
                return false;
            }

            if num == target {
                return true;
            }

            can_partition(num / 10, target - (num % 10)) ||
            can_partition(num / 100, target - (num % 100)) ||
            can_partition(num / 1000, target - (num % 1000))
        }
        (1..=n)
        .map(|i| (i, i * i))
        .filter(|&(a, b)| can_partition(b, a))
        .map(|(a, b)| b)
        .sum::<i32>()
    }
}
