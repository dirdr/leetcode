class Solution {
    public int[] replaceElements(int[] arr) {
        for (int i = 0; i < arr.length-1; i++) {
            int max = arr[i+1];
            for (int j = i+1; j < arr.length; j++) {
                max = arr[j] > max ? arr[j]: max;
            }
            arr[i] = max;
        }
        arr[arr.length-1] = -1;
        return arr;
    }
}

/**
 * Runtime: 156 ms, faster than 21.13% of Java online submissions for Replace Elements with Greatest Element on Right Side.
 * Memory Usage: 40.2 MB, less than 76.94% of Java online submissions for Replace Elements with Greatest Element on Right Side.
 */
 
