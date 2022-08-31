class Solution {
    public int countPrimes(int n) {
        
     if(n < 2)return 0;
        
      int count  = 0;
       boolean[] prime = new boolean[n] ;
       prime[0] = prime[1] = true;
        
        for(int i = 2 ; i * i < n ; i++){
          if(!prime[i]){
              for(int j = i * i ; j < n ; j += i ){
                  prime[j] = true;
              }
          }
        }
        
       for(boolean i : prime)if(!i)count++; 
        return count;
    }
}
