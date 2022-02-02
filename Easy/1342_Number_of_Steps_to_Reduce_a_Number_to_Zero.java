class Solution {
    public int numberOfSteps(int num) {
        int count = 0;
        int n = num;
        while (n > 0) {
            if (n % 2 == 0) n /= 2;
            else {
                n -= 1;
            }
            count++;
        }
        return count;
    }
}

/**
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Number of Steps to Reduce a Number to Zero.
* Memory Usage: 41.9 MB, less than 5.89% of Java online submissions for Number of Steps to Reduce a Number to Zero.
*/
