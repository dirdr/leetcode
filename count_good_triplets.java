class Solution {
    public int countGoodTriplets(int[] arr, int a, int b, int c) {
        boolean flag = true;
        int count = 0;
        for(int i = 0; i < arr.length; i++){
            for(int j = i+1; j < arr.length; j++){
                for(int k = j+1; k < arr.length; k++){
                    flag = true;
                    if(Math.abs(arr[i] - arr[j]) > a)
                        flag = false;
                    if(Math.abs(arr[j] - arr[k]) > b)
                        flag = false;
                    if(Math.abs(arr[i] - arr[k]) > c)
                        flag = false;
                    if(flag)
                        count++;
                         
                }
            }
        }
        return count;
    }
}
