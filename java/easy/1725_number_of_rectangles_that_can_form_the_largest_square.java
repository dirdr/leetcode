class Solution {
    public int countGoodRectangles(int[][] rectangles) {
        int count = 1;
        int max = rectangles[0][0] < rectangles[0][1] ? rectangles[0][0]: rectangles[0][1];
        for (int i = 1; i < rectangles.length; i++) {
            int l = rectangles[i][0];
            int w = rectangles[i][1];
            int comparable = l < w ? l: w;
            if (comparable > max) {
                max = comparable;
                count = 0;
            }
            if (l == max && w >= l || w == max && l >= w) {
                count++;
            }
        }
        return count;
    }
}
/**
 *Runtime: 1 ms, faster than 100.00% of Java online submissions for Number Of Rectangles That Can Form The Largest Square.
 *Memory Usage: 39.1 MB, less than 89.78% of Java online submissions for Number Of Rectangles That Can Form The Largest Square.
 */
