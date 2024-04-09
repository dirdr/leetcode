class Solution {
    public int isPrefixOfWord(String sentence, String searchWord) {
        String[] separated = sentence.split(" ");
        for (int i = 0; i < separated.length; i++) {
            if (separated[i].contains(searchWord) && separated[i].indexOf(searchWord) == 0) {
                return i+1;
            }
        }
        return -1;
    }
            
}
