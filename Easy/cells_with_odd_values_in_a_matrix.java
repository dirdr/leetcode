class Solution {
    public int oddCells(int n, int m, int[][] indices) {
        int count = 0;
        int[][] initialMatrix = new int[n][m];
        for(int i = 0; i < indices.length; i++){
            int temp_r = indices[i][0];
            int temp_c = indices[i][1];
            for(int j = 0; j < m; j++){
                initialMatrix[temp_r][j] += 1;
            }
            for(int k = 0; k < n; k++){
                initialMatrix[k][temp_c] += 1;
            }
            
        }
        for(int i = 0; i < n; i++){
            for(int j = 0; j < m; j++){
                if(initialMatrix[i][j] % 2 != 0)
                   count++; 
            }
        }
        return count;
    }
}
