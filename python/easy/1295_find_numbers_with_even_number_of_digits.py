class Solution:
    def findNumbers(self, nums: List[int]) -> int:
        return sum(1 for num in nums if (floor(log10(num)) + 1) % 2 == 0)
