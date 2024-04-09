class Solution {
    public int[] sortedSquares(int[] nums) {
        int l = 0;
        int r = nums.length-1;
        int[] answer = new int[nums.length];
        int i = 0;
        while (l <= r) {
            int left = nums[l]*nums[l];
            int right = nums[r]*nums[r];
            if (left > right) {
                answer[answer.length-i-1] = left;
                l++;
            } else {
                answer[answer.length-i-1] = right;
                r--;
            }
            i++;
        }
        return answer;
    }
}

/**
* Runtime: 1 ms, faster than 100.00% of Java online submissions for Squares of a Sorted Array.
* Memory Usage: 53.4 MB, less than 14.97% of Java online submissions for Squares of a Sorted Array.
*/
