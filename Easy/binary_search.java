class Solution {
    public int search(int[] nums, int target) {  
        int l = 0; 
        int r = nums.length-1;
        while (l <= r) {
            int mid = (l+r)/2;
            if (nums[mid] == target) return mid;
            if (target > nums[mid]) {
                l = mid+1;
            } else {
                r = mid-1;
            }
        }
        return -1;
    }
}

/**
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Binary Search.
* Memory Usage: 51.7 MB, less than 27.62% of Java online submissions for Binary Search.
*/
