class Solution:
    def countGood(self, nums: List[int], k: int) -> int:
        answer = 0
        goodPairs = 0
        freq = defaultdict(int)
        left = 0

        for right in range(len(nums)):
            goodPairs += freq[nums[right]]
            freq[nums[right]] += 1

            while goodPairs >= k:
                answer += len(nums) - right
                freq[nums[left]] -= 1
                goodPairs -= freq[nums[left]]
                left += 1

        return answer
