class Solution {
    public String sortSentence(String s) {
        
        StringBuilder sb = new StringBuilder();
        HashMap<Integer, String> map = new HashMap<>();
        String[] splited = s.split(" ");
        
        for (String str: splited) {
            int position = Character.getNumericValue(str.charAt(str.length()-1));
            map.put(position, str.substring(0, str.length()-1));
        }
        
        for (int i = 1; i <= splited.length; i++) {
            sb.append(map.get(i));
            if (i != splited.length)  sb.append(" ");
        }
      
        return sb.toString();
    }
}

/*
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Sorting the Sentence.
* Memory Usage: 40.3 MB, less than 77.37% of Java online submissions for Sorting the Sentence.
*/
