class Solution {
    public int minFlips(String target) {
        int count = 0;
        char state = '0'; //initial state
        for (char ch: target.toCharArray()) {
            if (ch != state) {
                state = ch;
                count++;
            }
        }
        return count;
    }
}

/**
 *Runtime: 3 ms, faster than 100.00% of Java online submissions for Bulb Switcher IV.
 *Memory Usage: 39.6 MB, less than 46.49% of Java online submissions for Bulb Switcher IV.
 */
