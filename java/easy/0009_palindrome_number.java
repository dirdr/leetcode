class Solution {
    public boolean isPalindrome(int x) {
        String str = String.valueOf(x);
        for (int i = 0; i < str.length()/2; i++) {
            char left = str.charAt(i);
            char right = str.charAt(str.length()-1-i);
            if (right != left) {
                return false;
            }
        }
        return true;
    }
}
/**
 *Runtime: 7 ms, faster than 71.56% of Java online submissions for Palindrome Number.
 *Memory Usage: 38.1 MB, less than 87.64% of Java online submissions for Palindrome Number.
 */
