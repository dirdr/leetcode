class Solution {
    public int addDigits(int num) {
        return 1 + (num - 1) % 9; //on retourne le reste de la division euclidienne par 9
    }
}
