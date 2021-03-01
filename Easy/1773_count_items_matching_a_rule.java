class Solution {
    public int countMatches(List<List<String>> items, String ruleKey, String ruleValue) {
        int count = 0;
        int indice = ruleKey.equals("type") ? 0: ruleKey.equals("color") ? 1: 2;
        for (List<String> list: items) {
            if (list.get(indice).equals(ruleValue)) count++;
        }  
        return count;
    }
}


/**
 *Runtime: 3 ms, faster than 100.00% of Java online submissions for Count Items Matching a Rule.
 *Memory Usage: 43.2 MB, less than 100.00% of Java online submissions for Count Items Matching a Rule.
 */

