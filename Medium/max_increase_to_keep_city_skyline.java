class Solution {
    public int maxIncreaseKeepingSkyline(int[][] grid) {
        ArrayList<Integer> topBottom = new ArrayList<Integer>();
        ArrayList<Integer> leftRight = new ArrayList<Integer>();
        //Left Right 
        for (int i = 0; i < grid.length; i++) {
            int max = grid[i][0];
            for (int j = 1; j < grid[0].length; j++) {
                if (grid[i][j] > max ) {
                    max = grid[i][j];
                }
            }
            leftRight.add(max);            
        }
        //Top Bottom
        for (int i = 0; i < grid[0].length; i++) {
            int max = grid[0][i];
            for (int j = 1; j < grid.length; j++) {
                if (grid[j][i] > max) {
                    max = grid[j][i];
                }
            }
            topBottom.add(max);
        }
        int result = 0;
        for (int i = 0; i < grid.length; i++) {
            for (int j = 0; j < grid[0].length; j++) {
                while (grid[i][j] < topBottom.get(j) && grid[i][j] < leftRight.get(i)) {
                    result++;
                    grid[i][j]++;
                }
            }
        }
        return result;
    }
}
