class Solution {
    public int numSpecial(int[][] mat) {
        if (mat.length == 0) {
            return 0;
        }
        int result = 0;
        for (int i = 0; i < mat.length; i++) {
            for (int j = 0; j < mat[i].length; j++) {
                if (mat[i][j] == 1 && checkerRaw(mat, i, j) == true && checkerColumn(mat, i, j) == true) {
                   result++;
                }
            }
        }
        return result;
    }
    
    
    public boolean checkerRaw(int[][] mat, int m, int n) {
        int count = 0;
        for (int i = 0; i < mat[m].length; i++) {
            if (mat[m][i] == 1) {
                count++;
            }
            if (count > 1) {
                return false;
            }
        }
        return true;
    }
    
    public boolean checkerColumn(int[][] mat, int m, int n) {
        int count = 0;
        for (int i = 0; i < mat.length; i++) {
            if (mat[i][n] == 1) {
                count++;
            }
            if (count > 1) {
                return false;
            }
        }
        return true;
    }
}
