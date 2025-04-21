class Solution:
    def numberOfArrays(self, differences: List[int], lower: int, upper: int) -> int:
        current = 0
        min_val, max_val = 0, 0
        for diff in differences:
            current += diff
            min_val = min(min_val, current)
            max_val = max(max_val, current)
        possible_starts = upper - lower + 1 - (max_val - min_val)
        return max(0, possible_starts)
