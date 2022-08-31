class Solution {
    public String[] findOcurrences(String text, String first, String second) {
        
        String[] splited = text.split(" ");
        ArrayList<String> list = new ArrayList<>();
        
        for (int i = 0; i < splited.length-2; i++) {
            String curr = splited[i];
            String aft = splited[i+1];
            if (curr.equals(first) && aft.equals(second)) {
                list.add(splited[i+2]);
            }
        }
        String[] ans = new String[list.size()];
        for (int i = 0; i < list.size(); i++) {
            ans[i] = list.get(i);
        }
        return ans;
    }
}
/**
 * Runtime: 0 ms, faster than 100.00% of Java online submissions for Occurrences After Bigram.
 * Memory Usage: 37 MB, less than 69.37% of Java online submissions for Occurrences After Bigram.
 */
