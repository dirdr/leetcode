class Solution {
    public int repeatedNTimes(int[] A) {
        int count = 0;
        int n = A.length/2;
        for(int i = 0; i < A.length - 1; i++){
            count = 0;
            for(int j = i; j < A.length; j++){
                if(A[i] == A[j])
                    count++;
            }
            if(count == n)
                return A[i];
        }
        return 0;
    }
}
