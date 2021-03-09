class Solution {
    public int[] relativeSortArray(int[] arr1, int[] arr2) {
        int[] result = new int[arr1.length];
        int index = 0;
        for (int i = 0; i < arr2.length; i++) {
            int curr = arr2[i];
            int count = 0;
            for (int j = 0; j < arr1.length; j++) {
                if (arr1[j] == curr) count++;
            }
            for (int j = index; j < index + count; j++) {
                result[j] = curr;
            }
            index+= count;
        }
        List<Integer> list = new ArrayList<Integer>();
        for (int i = 0; i < arr1.length; i++) {
            boolean flag = true;
            for (int j = 0; j < arr2.length; j++) {
                if (arr1[i] == arr2[j]) {
                    flag = false;
                    break;
                }
            }
            if (flag) {
                list.add(arr1[i]);
            }
        }
        Collections.sort(list);
        for (int i = 0; i < list.size(); i++) {
            result[index++] = list.get(i);
        }
        return result;
    }
}
/**
 * Runtime: 3 ms, faster than 43.26% of Java online submissions for Relative Sort Array.
 * Memory Usage: 37.4 MB, less than 99.49% of Java online submissions for Relative Sort Array.
 */



