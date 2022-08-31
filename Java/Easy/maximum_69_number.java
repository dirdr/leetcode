class Solution {
    public int maximum69Number (int num) {
        return Integer.parseInt(Integer.toString(num).replaceFirst("6", "9"));
    }
}
