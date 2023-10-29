class Solution {
    public int distributeCandies(int[] candyType) {
        HashSet<Integer> set = new HashSet();
        for (int candie: candyType) {
            set.add(candie);
        }
        int count = 0;
        for (int candie: set) {
            if (count < candyType.length/2) {
                count++;
            } else {
                return count;
            }
        }
        return count;
    }
}
