class Solution:
    def countSubarrays(self, nums: List[int], k: int) -> int:
        n = len(nums)
        left, freq, answer = 0, 0, 0
        max_value = max(nums)
        for right in range(n):
            if nums[right] == max_value:
                freq += 1
            while freq >= k:
                answer += n - right
                if nums[left] == max_value:
                    freq -= 1
                left += 1
        return answer
