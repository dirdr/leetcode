class Solution {
    public int removeElement(int[] nums, int val) {
        
        
        if (nums.length == 0) return 0;
        
        int encountered = 0;
        int i = 0;
        int len = nums.length;
        int change_pointer = len-1;
        
        for(int num: nums) if (num == val) encountered++;
        
        while (change_pointer > i) {
            if (nums[i] == val) {
                if (nums[change_pointer] == val) {
                    change_pointer--;
                } else {
                    swap(nums, i, change_pointer);
                    i++;
                }
            } else {
                i++;
            }
        }
        return len-encountered;  
    }
    
    public void swap(int[] nums, int a, int b) {
        int temp = nums[a];
        nums[a] = nums[b];
        nums[b] = temp;
    }
}

/*
* Runtime: 1 ms, faster than 28.65% of Java online submissions for Remove Element.
* Memory Usage: 43 MB, less than 8.49% of Java online submissions for Remove Element.
*/
