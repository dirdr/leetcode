class Solution:
    def addBinary(self, a: str, b: str) -> str:
        largest, smallest = (a, b) if len(a) > len(b) else (b, a)
        answer = []
        smallest = smallest.zfill(len(largest))
        carry = 0
        for first, second in zip(largest[::-1], smallest[::-1]):
            val = int(first) + int(second) + carry
            carry = 1 if val >= 2 else 0
            answer.append(str(val % 2))

        if carry:
            answer.append('1')
            
        return ''.join(reversed(answer))
