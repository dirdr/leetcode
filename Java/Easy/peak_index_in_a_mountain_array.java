class Solution {
    public int peakIndexInMountainArray(int[] arr) {
        int max = arr[0];
        int maxIndice = 0;
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] > max) {
                max = arr[i];
                maxIndice = i;
            }
        }
        return maxIndice;
    }
}
