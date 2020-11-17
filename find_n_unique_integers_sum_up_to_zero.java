class Solution {
    public int[] sumZero(int n) {
        int[] result = new int[n];
        for(int i = 0; i < result.length-1; i+=2){
            result[i] = n;
            result[i+1] = -n;
            n--;
        }
        return result;    
    }
}
