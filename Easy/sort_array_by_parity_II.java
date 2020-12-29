class Solution {
    public int[] sortArrayByParityII(int[] A) {
        int[] answer = new int[A.length];
        int indiceEven = 0;
        int indiceOdd = 1;
        for (int i = 0; i < A.length; i++) {
            if (A[i] % 2 == 0) {
                answer[indiceEven] = A[i];
                indiceEven += 2;
            }
            if (A[i] % 2 != 0) {
                answer[indiceOdd] = A[i];
                indiceOdd += 2;
            }
        }
        return answer;
    }
}
