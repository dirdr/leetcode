class Solution {
    public int[] sortArrayByParity(int[] A) {
        int[] result = new int[A.length];
        int temp = 0;
        for(int i = 0; i < result.length; i++){
            if(A[i] % 2 == 0){
                result[temp] = A[i];
                temp++;
            }    
        }
        for(int i = 0; i < result.length; i++){
            if(A[i] % 2 != 0){
                result[temp] = A[i];
                temp++;
            }
        }
        return result;
    }
}
