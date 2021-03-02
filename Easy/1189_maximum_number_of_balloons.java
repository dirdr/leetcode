class Solution {
    public int maxNumberOfBalloons(String text) {
        HashMap<Character, Integer> map = new HashMap<Character, Integer>();
        for (char ch: text.toCharArray()) {
            if ("balloon".contains(String.valueOf(ch))) {
                map.put(ch, map.getOrDefault(ch, 0) +1);
            }
        }
        int min = text.length()/7;
        for (char key: map.keySet()) {
            if (key == 'l' || key == 'o') {
                min = map.get(key)/2 < min ? map.get(key)/2: min;
            } else {
                min = map.get(key) < min ? map.get(key): min;
            }
        }
        return min;
    }
}
/**
 *Runtime: 5 ms, faster than 37.06% of Java online submissions for Maximum Number of Balloons.
 *Memory Usage: 39.4 MB, less than 10.55% of Java online submissions for Maximum Number of Balloons.
 */

