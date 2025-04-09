class Solution:
    def minOperations(self, nums: List[int], k: int) -> int:
        answer, unique = 0, set(nums)
    
        for el in unique:
            if el > k:
                answer += 1
            if el < k:
                return -1

        return answer
