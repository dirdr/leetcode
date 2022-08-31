class Solution {
    public String restoreString(String s, int[] indices) {
        char[] tab = new char[s.length()];
        for(int i = 0; i < s.length(); i++){
            tab[i] = s.charAt(i);
        }
        char[] result = new char[tab.length];
        for(int i = 0; i < result.length; i ++){
            result[indices[i]] = tab[i];
        }
        return String.valueOf(result);
        
    }
}
