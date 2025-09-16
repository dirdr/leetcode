impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let temp = a % b;
                a = b;
                b = temp;
            }
            a
        } 
        fn lcm(a: i32, b: i32) -> i32 {
            a / gcd(a, b) * b
        }
        let mut result = Vec::new();
        for num in nums {
            result.push(num);
            while result.len() >= 2 {
                let len = result.len();
                let last = result[len - 1];
                let second_last = result[len - 2];
                if gcd(second_last, last) > 1 {
                    result.pop();
                    result.pop();
                    result.push(lcm(second_last, last));
                } else {
                    break;
                }
            }
        }
        result
    }
}
