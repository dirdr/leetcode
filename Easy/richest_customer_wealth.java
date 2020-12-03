class Solution {
    public int maximumWealth(int[][] accounts) {
        int[] result = new int[accounts.length];
        for(int i = 0; i < accounts.length; i++){
            int tempWealth = 0;
            for(int j = 0; j < accounts[i].length; j++){
                tempWealth += accounts[i][j];
            }
            result[i] = tempWealth;   
        }
        int max = result[0];
        for(int i = 1; i < result.length; i++){
            if(result[i] > max)
                max = result[i];
        }
        return max;
    }
}
