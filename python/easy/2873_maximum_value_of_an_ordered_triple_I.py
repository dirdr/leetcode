class Solution:
    def maximumTripletValue(self, nums: List[int]) -> int:
        n = len(nums)
        prefix, suffix = [0] * n, [0] * n
        prefix[0] = nums[0]
        suffix[n - 1] = nums[n - 1]

        for i in range(1, n):
            prefix[i] = max(prefix[i - 1], nums[i])
            
        for i in range(n - 2, 1, -1):
            suffix[i] = max(suffix[i + 1], nums[i])
        print(suffix)

        answer = 0
        for i in range(1, n - 1):
            triplet = (prefix[i - 1] - nums[i]) * suffix[i + 1]
            answer = max(answer, triplet)

        return answer
