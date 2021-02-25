class Solution {
    public String mergeAlternately(String word1, String word2) {
        StringBuilder sb = new StringBuilder();
        String longest = word1.length() > word2.length() ? word1 : word2;
        String shortest = word1.length() < word2.length() ? word1 : word2;
        String rest = longest.substring(shortest.length());
        for (int i = 0; i < shortest.length(); i++) {
            sb.append(word1.charAt(i));
            sb.append(word2.charAt(i));
        }
        for (char ch: rest.toCharArray()) {
            sb.append(ch);
        }
        String result = sb.toString();   
        return result;
    }
