impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let helper: Vec<_> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        if helper.len() <= 1 {
            return true;
        }
        let (mut left, mut right) = (0 as usize, helper.len() - 1);
        while left <= right {
            if helper[left] != helper[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
