class Solution {
    public String firstPalindrome(String[] words) {
        for (String word: words) {
            boolean is_pal = true;
            if (word.length() == 1) return word;
            for (int i = 0; i < word.length()/2; i++) {
                if (word.charAt(i) != word.charAt(word.length()-i-1)) {
                    is_pal = false;
                    break;
                }
            }
            if (is_pal) return word;
        }
        return "";
    }
}
