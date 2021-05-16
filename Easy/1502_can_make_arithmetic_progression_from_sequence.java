class Solution {
    public boolean canMakeArithmeticProgression(int[] arr) {
        Arrays.sort(arr);
        int step = arr[1] - arr[0];
        for (int i = 1; i < arr.length-1; i++) {
            if (arr[i+1] - arr[i] != step) {
                return false;
            }
        }
        return true;
    }
}

/**
 * Runtime: 1 ms, faster than 98.98% of Java online submissions for Can Make Arithmetic Progression From Sequence.
 * Memory Usage: 38.1 MB, less than 81.66% of Java online submissions for Can Make Arithmetic Progression From Sequence.
 */
