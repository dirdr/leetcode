class Solution {
    public int[] createTargetArray(int[] nums, int[] index) {
        
        ArrayList<Integer> answer = new ArrayList();
        
        for (int i = 0; i < index.length; i++) {
            answer.add(index[i], nums[i]);
        }
        int[] target = new int[index.length];
        for (int i = 0; i < answer.size(); i++) {
            target[i] = answer.get(i);
        }
        return target;
   }
}
