class Solution:
    def numRabbits(self, answers: List[int]) -> int:
        freq = Counter(answers)
        answer = 0
        for k, v in freq.items():
            answer += ceil(v / (k + 1)) * (k + 1)
        return answer
