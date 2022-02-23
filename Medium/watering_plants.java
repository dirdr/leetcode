class Solution {
    public int wateringPlants(int[] plants, int capacity) {
        int count = 0;
        int current_capacity = capacity;
        for (int i = 0; i < plants.length; i++) {
            count++;
            if (current_capacity >= plants[i]) {   
                current_capacity -= plants[i];
            } else { // need to go to the river
                count--;
                count += (2 * i) + 1;
                current_capacity = capacity-plants[i];
            }
        }
        return count;
    }
}

/**
* Runtime: 0 ms, faster than 100.00% of Java online submissions for Watering Plants.
* Memory Usage: 41.2 MB, less than 39.92% of Java online submissions for Watering Plants.
*/
