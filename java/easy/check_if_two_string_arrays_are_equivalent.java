class Solution {
    public boolean arrayStringsAreEqual(String[] word1, String[] word2) {
        String temp1 = String.join("", word1);
        String temp2 = String.join("", word2);
        if(temp1.equals(temp2))
            return true;
        return false;
    }
}
