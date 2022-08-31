class Solution {
    public List<Integer> luckyNumbers (int[][] matrix) {
        List<Integer> list = new ArrayList<Integer>();
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                List<Integer> sortedList1 = new ArrayList<Integer>();
                List<Integer> sortedList2 = new ArrayList<Integer>();
                for (int k = 0; k < matrix.length; k++) {
                    sortedList1.add(matrix[k][j]);
                }
                for (int k = 0; k < matrix[i].length; k++) {
                    sortedList2.add(matrix[i][k]);
                }
                if (Collections.max(sortedList1) == matrix[i][j] && Collections.min(sortedList2) == matrix[i][j]) {
                    list.add(matrix[i][j]);
                }
            }
        }
        return list;
    }
}
