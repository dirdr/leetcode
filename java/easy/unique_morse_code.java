class Solution {
    public int uniqueMorseRepresentations(String[] words) {
        String[] fullTable = {".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."};
        
        Set<String> set = new HashSet<>();
        for (int i = 0; i < words.length; i++) {
            StringBuilder sb = new StringBuilder("");
            char[] str = words[i].toCharArray();
            for (char ch: str) sb.append(fullTable[ch-97]); //ascii table 
            set.add(sb.toString());
        }
        return set.size();
    }
}
