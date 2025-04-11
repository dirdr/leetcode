class Solution:
    def countSymmetricIntegers(self, low: int, high: int) -> int:
        count = 0
        for num in range(low, high + 1):
            digits = str(num)
            if len(digits) % 2 != 0:
                continue

            half_length = len(digits) // 2
            left_sum = sum(int(digit) for digit in digits[:half_length])
            right_sum = sum(int(digit) for digit in digits[half_length:])
            if left_sum == right_sum:
                count += 1
        return count
