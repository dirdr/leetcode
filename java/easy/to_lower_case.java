class Solution {
    public String toLowerCase(String str) {
        char[] result = str.toCharArray();
        for(int i = 0; i < result.length; i++){
            result[i] = Character.toLowerCase(result[i]);
        }
        return String.valueOf(result);
    }
}
