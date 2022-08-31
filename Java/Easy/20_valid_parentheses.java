class Solution {
    
    public boolean isValid(String s) {
        
        HashMap<Character, Character> map = new HashMap<>();
        
        map.put(')', '(');
        map.put('}', '{');
        map.put(']', '[');
        
        Stack<Character> stack = new Stack<>();
        
        for (char ch: s.toCharArray()) {
            if (map.get(ch) != null) { //closing parenthese
                if (stack.size() != 0 && stack.peek() == map.get(ch)) {
                    stack.pop();
                } else {
                    return false;
                }
            } else {
                stack.push(ch);
            }
        }
        
        return stack.size() == 0;
        
    }  
}

/*
* Runtime: 2 ms, faster than 80.43% of Java online submissions for Valid Parentheses.
* Memory Usage: 42.2 MB, less than 21.68% of Java online submissions for Valid Parentheses.
*/
