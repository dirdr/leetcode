class Solution {
    public List<Integer> luckyNumbers (int[][] matrix) {
        List<Integer> list = new ArrayList<Integer>();
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[i].length; j++) {
                if (isMinRaw(matrix, i, j) == true && isMaxColumn(matrix, i, j) == true) {
                    list.add(matrix[i][j]);
                    
                }
            }
        }
        return list;
    }
    
    public boolean isMinRaw(int[][] matrix, int m, int n) {
        int min = matrix[m][n];
        for (int i = 0; i < matrix[m].length; i++) {
            if (matrix[m][i] < min) {
                return false;
            }
        }
        return true;
    }
    public boolean isMaxColumn(int[][] matrix, int m, int n) {
        int max = matrix[m][n];
        for (int i = 0; i < matrix.length; i++) {
            if (matrix[i][n] > max) {
                return false;
            }
        }
        return true;
    }
}
