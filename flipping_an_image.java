class Solution {
    public int[][] flipAndInvertImage(int[][] A) {
            for(int i = 0; i < A.length; i++){
                for(int j = 0; j < A[i].length/2; j++){
                    int temp = A[i][j];
                    A[i][j] = A[i][A.length-j-1];
                    A[i][A.length-j-1] = temp;
                }
                for(int k = 0; k < A[i].length; k++){
                    if(A[i][k] == 0)
                        A[i][k] = 1;
                    else
                        A[i][k] = 0;
                }
            }
        return A;
    }
        
}
