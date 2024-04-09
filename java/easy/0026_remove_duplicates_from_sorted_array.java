class Solution {
    public int removeDuplicates(int[] nums) {
        
        int duplicate_pointer = 0;
        int duplicate_count = 0;
        
        HashSet<Integer> set = new HashSet<>();
        for (int i = 0; i < nums.length; i++) {
            if (!set.contains(nums[i])) {
                set.add(nums[i]);
                swap(nums, i, duplicate_pointer);
                duplicate_pointer++;
            } else {
                duplicate_count++;
            }
        }
        return nums.length - duplicate_count;
    }
    
    public void swap(int nums[], int a, int b) {
        int temp = nums[a];
        nums[a] = nums[b];
        nums[b] = temp;
    }
}

/*
* Runtime: 4 ms, faster than 11.66% of Java online submissions for Remove Duplicates from Sorted Array.
* Memory Usage: 48.8 MB, less than 5.00% of Java online submissions for Remove Duplicates from Sorted Array.
*/
