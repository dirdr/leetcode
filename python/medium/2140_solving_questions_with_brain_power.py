class Solution:
    def mostPoints(self, questions: List[List[int]]) -> int:
        n = len(questions)
        dp = [0] * (n + 1)
        for i in range(n - 1, -1, -1):
            points, jump = questions[i]
            skip = dp[i + 1]
            next_question_idx = min(n, i + jump + 1)
            solve = points + dp[next_question_idx]
            dp[i] = max(skip, solve)
        return dp[0]
