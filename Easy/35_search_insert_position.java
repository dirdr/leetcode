class Solution {
    public int searchInsert(int[] nums, int target) {
        
        int l = 0;
        int r = nums.length-1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (nums[mid] == target) return mid;
            if (target > nums[mid]) {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
            i = mid;
        }
    }
}


/** 
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Search Insert Position.
* Memory Usage: 40.8 MB, less than 10.45% of Java online submissions for Search Insert Position.
*/
