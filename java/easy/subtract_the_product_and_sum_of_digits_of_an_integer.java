class Solution {
    public int subtractProductAndSum(int n) {
        char[] charTab = String.valueOf(n).toCharArray();
        int[] intTab = new int[charTab.length];
        for(int i = 0; i < charTab.length; i++){
            intTab[i] = Character.getNumericValue(charTab[i]);
        }
        int sum = 0;
        int product = intTab[0];
        for(int i = 0; i < intTab.length; i++){
            sum += intTab[i];
        }
        for(int i = 1; i < intTab.length; i++){
            product *= intTab[i];
        }
        return product-sum;
    }
}
