class Solution {
    public int maxPower(String s) {
        int power, maxPower;
        power = maxPower = 1;
        for (int i = 0; i < s.length()-1; i++) {
            char curr = s.charAt(i);
            char after = s.charAt(i+1);
            if (curr == after) {
                power++;
            } else {
                power = 1;
            }
            maxPower = power > maxPower ? power : maxPower;
        }
        return maxPower;
    }
}
